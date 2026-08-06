use std::path::Path;
use std::sync::Arc;

use dashmap::DashMap;

use crate::activity::{ActivityContext, ActivityRegistry};
use crate::definition::WorkflowDefinition;
use crate::error::Result;
use crate::error::WorkflowError;
use crate::event::{WorkflowEvent, timestamp_now};
use crate::state::{StepState, StepStatus, WorkflowState, WorkflowStatus};
use crate::store::WorkflowStore;

type ActiveHandle = tokio::task::JoinHandle<()>;

pub struct WorkflowEngine {
    store: WorkflowStore,
    activities: ActivityRegistry,
    active_runs: DashMap<String, ActiveHandle>,
}

impl WorkflowEngine {
    pub fn open(path: &Path) -> sled::Result<Self> {
        let store = WorkflowStore::open(path)?;
        Ok(Self {
            store,
            activities: ActivityRegistry::default(),
            active_runs: DashMap::new(),
        })
    }

    pub fn register_activity<A: crate::activity::Activity + 'static>(&mut self, activity: A) {
        self.activities.register(activity);
    }

    pub fn activities(&self) -> &ActivityRegistry {
        &self.activities
    }

    pub fn store(&self) -> &WorkflowStore {
        &self.store
    }

    pub async fn start(
        &self,
        definition: &(dyn WorkflowDefinition + Send + Sync),
        agent_id: Option<String>,
    ) -> Result<String> {
        let name = definition.name().to_string();
        let steps = definition.steps();

        if steps.is_empty() {
            return Err(WorkflowError::Validation("workflow has no steps".into()));
        }

        let mut state = WorkflowState::new(&name, agent_id.clone());
        state.step_states = steps
            .iter()
            .map(|s| StepState::new(&s.id, s.input.clone()))
            .collect();

        self.store.save_workflow(&state)?;
        self.store.index_by_status(&state.id, "Running")?;
        if let Some(ref agent) = agent_id {
            self.store.index_by_agent(agent, &state.id)?;
        }

        let event = WorkflowEvent::WorkflowStarted {
            id: state.id.clone(),
            definition: name,
            timestamp: timestamp_now(),
        };
        self.store.append_event(&state.id, &event)?;

        let wf_id = state.id.clone();
        self.spawn_loop(wf_id, steps, agent_id).await
    }

    pub async fn start_yaml(&self, yaml: &str, agent_id: Option<String>) -> Result<String> {
        let parsed = crate::definition::YamlWorkflow::from_yaml(yaml)
            .map_err(|e| WorkflowError::Validation(format!("invalid YAML: {}", e)))?;
        let def = parsed.to_definition();
        self.start(&def, agent_id).await
    }

    async fn spawn_loop(
        &self,
        wf_id: String,
        steps: Vec<crate::definition::WorkflowStep>,
        agent_id: Option<String>,
    ) -> Result<String> {
        let store_events = self.store.db().open_tree("workflow_events")?;
        let store_workflows = self.store.db().open_tree("workflows")?;
        let activities = self.activities_ref_snapshot();
        let wf_id_for_result = wf_id.clone();

        let handle = tokio::spawn(async move {
            loop {
                let key = wf_id.as_bytes();
                let raw = store_workflows.get(key).unwrap();
                if raw.is_none() {
                    return;
                }
                let mut state: WorkflowState = serde_json::from_slice(&raw.unwrap()).unwrap();
                let status = state.status.clone();
                let idx = state.current_step;

                match status {
                    WorkflowStatus::Paused => {
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        continue;
                    }
                    WorkflowStatus::Failed(_)
                    | WorkflowStatus::Completed
                    | WorkflowStatus::Cancelled => {
                        return;
                    }
                    _ => {}
                }

                if idx >= steps.len() {
                    let ev = WorkflowEvent::WorkflowCompleted {
                        id: wf_id.clone(),
                        timestamp: timestamp_now(),
                    };
                    store_events
                        .insert(
                            composite_key(
                                &wf_id,
                                store_events.scan_prefix(wf_id.as_bytes()).count() as u64,
                            ),
                            serde_json::to_vec(&ev).unwrap(),
                        )
                        .unwrap();
                    state.status = WorkflowStatus::Completed;
                    state.mark_updated();
                    store_workflows
                        .insert(key, serde_json::to_vec(&state).unwrap())
                        .unwrap();
                    return;
                }

                let step = &steps[idx];
                let step_state = &mut state.step_states[idx];

                if step_state.is_terminal() {
                    state.current_step = idx + 1;
                    store_workflows
                        .insert(key, serde_json::to_vec(&state).unwrap())
                        .unwrap();
                    continue;
                }

                if step.requires_approval && matches!(step_state.status, StepStatus::Pending) {
                    step_state.status = StepStatus::Paused;
                    state.status = WorkflowStatus::Paused;
                    state.mark_updated();
                    store_workflows
                        .insert(key, serde_json::to_vec(&state).unwrap())
                        .unwrap();

                    let ev = WorkflowEvent::WorkflowPaused {
                        id: wf_id.clone(),
                        step: step.id.clone(),
                        reason: "awaiting approval".into(),
                        timestamp: timestamp_now(),
                    };
                    store_events
                        .insert(
                            composite_key(
                                &wf_id,
                                store_events.scan_prefix(wf_id.as_bytes()).count() as u64,
                            ),
                            serde_json::to_vec(&ev).unwrap(),
                        )
                        .unwrap();
                    continue;
                }

                let attempt = step_state.attempt;

                let ev = WorkflowEvent::StepStarted {
                    id: wf_id.clone(),
                    step: step.id.clone(),
                    attempt,
                    timestamp: timestamp_now(),
                };
                store_events
                    .insert(
                        composite_key(
                            &wf_id,
                            store_events.scan_prefix(wf_id.as_bytes()).count() as u64,
                        ),
                        serde_json::to_vec(&ev).unwrap(),
                    )
                    .unwrap();

                let activity = match activities.get(&step.activity) {
                    Some(a) => a,
                    None => {
                        let ev_err = WorkflowEvent::StepFailed {
                            id: wf_id.clone(),
                            step: step.id.clone(),
                            error: format!("unknown activity: {}", step.activity),
                            attempt,
                            timestamp: timestamp_now(),
                        };
                        store_events
                            .insert(
                                composite_key(
                                    &wf_id,
                                    store_events.scan_prefix(wf_id.as_bytes()).count() as u64,
                                ),
                                serde_json::to_vec(&ev_err).unwrap(),
                            )
                            .unwrap();
                        state.status =
                            WorkflowStatus::Failed(format!("unknown activity: {}", step.activity));
                        store_workflows
                            .insert(key, serde_json::to_vec(&state).unwrap())
                            .unwrap();
                        return;
                    }
                };

                let ctx = ActivityContext {
                    workflow_id: wf_id.clone(),
                    step_id: step.id.clone(),
                    attempt,
                    agent_id: agent_id.clone(),
                };

                let timeout = std::time::Duration::from_millis(step.timeout_ms);
                let result =
                    tokio::time::timeout(timeout, activity.execute(step.input.clone(), ctx)).await;

                match result {
                    Ok(Ok(output)) => {
                        step_state.status = StepStatus::Completed;
                        step_state.output = Some(output.clone());
                        step_state.completed_at = Some(timestamp_now());

                        let ev = WorkflowEvent::StepCompleted {
                            id: wf_id.clone(),
                            step: step.id.clone(),
                            output,
                            timestamp: timestamp_now(),
                        };
                        store_events
                            .insert(
                                composite_key(
                                    &wf_id,
                                    store_events.scan_prefix(wf_id.as_bytes()).count() as u64,
                                ),
                                serde_json::to_vec(&ev).unwrap(),
                            )
                            .unwrap();

                        state.current_step = idx + 1;
                        state.mark_updated();
                        store_workflows
                            .insert(key, serde_json::to_vec(&state).unwrap())
                            .unwrap();
                    }
                    Ok(Err(e)) => {
                        let retry = step.retry.can_retry(attempt);
                        if retry {
                            let backoff = step.retry.backoff_ms(attempt + 1);
                            step_state.attempt = attempt + 1;

                            let ev = WorkflowEvent::StepRetrying {
                                id: wf_id.clone(),
                                step: step.id.clone(),
                                attempt: attempt + 1,
                                backoff_ms: backoff,
                                timestamp: timestamp_now(),
                            };
                            store_events
                                .insert(
                                    composite_key(
                                        &wf_id,
                                        store_events.scan_prefix(wf_id.as_bytes()).count() as u64,
                                    ),
                                    serde_json::to_vec(&ev).unwrap(),
                                )
                                .unwrap();

                            state.mark_updated();
                            store_workflows
                                .insert(key, serde_json::to_vec(&state).unwrap())
                                .unwrap();
                            drop(state);
                            tokio::time::sleep(std::time::Duration::from_millis(backoff)).await;
                            continue;
                        } else {
                            step_state.status = StepStatus::Failed(e.to_string());
                            let ev = WorkflowEvent::StepFailed {
                                id: wf_id.clone(),
                                step: step.id.clone(),
                                error: e.to_string(),
                                attempt,
                                timestamp: timestamp_now(),
                            };
                            store_events
                                .insert(
                                    composite_key(
                                        &wf_id,
                                        store_events.scan_prefix(wf_id.as_bytes()).count() as u64,
                                    ),
                                    serde_json::to_vec(&ev).unwrap(),
                                )
                                .unwrap();

                            match step.on_failure {
                                crate::definition::FailurePolicy::Halt => {
                                    state.status = WorkflowStatus::Failed(e.to_string());
                                    store_workflows
                                        .insert(key, serde_json::to_vec(&state).unwrap())
                                        .unwrap();
                                    return;
                                }
                                crate::definition::FailurePolicy::Continue => {
                                    step_state.status = StepStatus::Skipped;
                                    state.current_step = idx + 1;
                                    state.mark_updated();
                                    store_workflows
                                        .insert(key, serde_json::to_vec(&state).unwrap())
                                        .unwrap();
                                }
                            }
                        }
                    }
                    Err(_timeout) => {
                        let retry = step.retry.can_retry(attempt);
                        if retry {
                            step_state.attempt = attempt + 1;
                            let backoff = step.retry.backoff_ms(attempt + 1);
                            state.mark_updated();
                            store_workflows
                                .insert(key, serde_json::to_vec(&state).unwrap())
                                .unwrap();
                            drop(state);
                            tokio::time::sleep(std::time::Duration::from_millis(backoff)).await;
                            continue;
                        } else {
                            step_state.status = StepStatus::Failed("timeout".into());
                            let ev = WorkflowEvent::StepFailed {
                                id: wf_id.clone(),
                                step: step.id.clone(),
                                error: format!("timed out after {}ms", step.timeout_ms),
                                attempt,
                                timestamp: timestamp_now(),
                            };
                            store_events
                                .insert(
                                    composite_key(
                                        &wf_id,
                                        store_events.scan_prefix(wf_id.as_bytes()).count() as u64,
                                    ),
                                    serde_json::to_vec(&ev).unwrap(),
                                )
                                .unwrap();
                            state.status =
                                WorkflowStatus::Failed(format!("timed out at step {}", step.id));
                            store_workflows
                                .insert(key, serde_json::to_vec(&state).unwrap())
                                .unwrap();
                            return;
                        }
                    }
                }
            }
        });

        self.active_runs.insert(wf_id_for_result.clone(), handle);
        Ok(wf_id_for_result)
    }

    fn activities_ref_snapshot(
        &self,
    ) -> std::collections::HashMap<String, Arc<dyn crate::activity::Activity>> {
        let mut map = std::collections::HashMap::new();
        for name in self.activities.list() {
            if let Some(a) = self.activities.get(&name) {
                map.insert(name, a);
            }
        }
        map
    }

    pub async fn resume(&self, workflow_id: &str, signal: Signal) -> Result<()> {
        let mut state = self.load_state(workflow_id)?;

        match signal {
            Signal::Approve => {
                if state.status != WorkflowStatus::Paused {
                    return Err(WorkflowError::SignalRejected {
                        reason: "workflow is not paused".into(),
                    });
                }
                state.status = WorkflowStatus::Running;
                state.mark_updated();
                self.store.save_workflow(&state)?;
                self.store.remove_status_index(workflow_id, "Paused")?;
                self.store.index_by_status(workflow_id, "Running")?;

                let ev = WorkflowEvent::WorkflowResumed {
                    id: workflow_id.into(),
                    by: "human_approval".into(),
                    timestamp: timestamp_now(),
                };
                self.store.append_event(workflow_id, &ev)?;
            }
            Signal::Cancel => {
                state.status = WorkflowStatus::Cancelled;
                state.mark_updated();
                self.store.save_workflow(&state)?;
                self.store.remove_status_index(workflow_id, "Running")?;

                let ev = WorkflowEvent::WorkflowCancelled {
                    id: workflow_id.into(),
                    timestamp: timestamp_now(),
                };
                self.store.append_event(workflow_id, &ev)?;
            }
        }

        Ok(())
    }

    pub fn state(&self, workflow_id: &str) -> Result<WorkflowState> {
        self.load_state(workflow_id)
    }

    pub fn list(&self) -> Result<Vec<WorkflowState>> {
        let tree = self.store.workflows_tree()?;
        let mut states = Vec::new();
        for item in tree.iter() {
            let (_, v) = item?;
            let state: WorkflowState = serde_json::from_slice(&v)?;
            states.push(state);
        }
        Ok(states)
    }

    pub fn list_by_agent(&self, agent_id: &str) -> Result<Vec<WorkflowState>> {
        let ids = self.store.list_by_agent(agent_id)?;
        let mut states = Vec::new();
        for id in ids {
            if let Some(state) = self.store.load_workflow(&id)? {
                states.push(state);
            }
        }
        Ok(states)
    }

    pub fn replay(&self, workflow_id: &str) -> Result<Vec<WorkflowEvent>> {
        self.store.load_events(workflow_id)
    }

    pub fn cancel(&self, workflow_id: &str) -> Result<()> {
        let mut state = self.load_state(workflow_id)?;
        state.status = WorkflowStatus::Cancelled;
        state.mark_updated();
        self.store.save_workflow(&state)?;
        Ok(())
    }

    pub async fn recover_on_boot(&self) -> Result<()> {
        let running = self.store.scan_by_status("Running")?;
        let paused = self.store.scan_by_status("Paused")?;
        let to_recover: Vec<String> = running.into_iter().chain(paused).collect();

        if to_recover.is_empty() {
            return Ok(());
        }

        for wf_id in to_recover {
            let state = match self.load_state(&wf_id) {
                Ok(s) => s,
                Err(_) => continue,
            };

            let events = self.store.load_events(&wf_id)?;
            if !events.is_empty() {
                let mut recovered = state.clone();
                recovered.step_states = state
                    .step_states
                    .iter()
                    .map(|_| StepState::new("", serde_json::Value::Null))
                    .collect();
            }

            if matches!(state.status, WorkflowStatus::Paused) {
                self.store.remove_status_index(&wf_id, "Paused")?;
                self.store.index_by_status(&wf_id, "Running")?;
            }
        }

        Ok(())
    }

    fn load_state(&self, workflow_id: &str) -> Result<WorkflowState> {
        self.store
            .load_workflow(workflow_id)?
            .ok_or_else(|| WorkflowError::NotFound(workflow_id.into()))
    }

    pub fn flush(&self) -> sled::Result<usize> {
        self.store.flush()
    }
}

fn composite_key(prefix: &str, seq: u64) -> Vec<u8> {
    let mut key = Vec::with_capacity(prefix.len() + 9);
    key.extend_from_slice(prefix.as_bytes());
    key.push(0);
    key.extend_from_slice(&seq.to_be_bytes());
    key
}

#[derive(Debug, Clone)]
pub enum Signal {
    Approve,
    Cancel,
}
