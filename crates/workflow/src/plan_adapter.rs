//! Adapter between the execution planner and the durable workflow engine.
//!
//! An [`ExecutionPlan`](bastion_core::execution::ExecutionPlan) produced by
//! `crates/core/src/execution` is a chain-agnostic set of ordered legs. This
//! module turns it into a [`WorkflowDefinition`] the workflow engine can run
//! durably: every leg becomes a `simulate -> [approve] -> settle` group,
//! compensation is exposed as its own workflow, and the terminal outcomes are
//! written back onto the plan so the `AtomicityManager` can reconcile it.

use std::collections::HashMap;

use async_trait::async_trait;
use bastion_core::execution::AtomicityManager;
use bastion_core::execution::plan::{ExecutionPlan, Leg, LegStatus, PlanStatus};
use bastion_core::transaction::Chain;
use uuid::Uuid;

use crate::definition::{FailurePolicy, WorkflowDefinition, WorkflowStep};
use crate::retry::RetryPolicy;
use crate::state::{StepStatus, WorkflowState};

/// Options that control how a plan becomes a workflow.
#[derive(Debug, Clone)]
pub struct PlanAdapterOptions {
    /// Whether to insert a human-approval step before settling each leg.
    pub require_approval: bool,
    /// Retry policy applied to simulation and settlement steps.
    pub retry: RetryPolicy,
    /// Per-step execution timeout in milliseconds.
    pub timeout_ms: u64,
    /// Timeout for approval steps (humans are slow).
    pub approval_timeout_ms: u64,
}

impl Default for PlanAdapterOptions {
    fn default() -> Self {
        Self {
            require_approval: true,
            retry: RetryPolicy {
                max_attempts: 3,
                ..RetryPolicy::default()
            },
            timeout_ms: 60_000,
            approval_timeout_ms: 3_600_000,
        }
    }
}

/// The simulation activity to use for a given chain.
fn simulate_activity(chain: Chain) -> &'static str {
    match chain {
        Chain::Solana => "simulate",
        // Every EVM chain (Ethereum, Base, Polygon, Arbitrum, Celo, ZkSync,
        // Robinhood) goes through the EVM simulator.
        _ => "simulate_evm",
    }
}

/// First component of a UUID, used to build readable, stable step ids.
fn short(id: &Uuid) -> String {
    id.to_string()[..8].to_string()
}

/// A [`WorkflowDefinition`] generated from an [`ExecutionPlan`].
///
/// Each leg maps to a group of steps: `simulate`, an optional `approve`
/// (human-in-the-loop), and `settle`. Every step carries its `leg_id` so the
/// outcomes can be written back onto the plan with [`sync_plan_from_workflow`].
#[derive(Debug, Clone)]
pub struct PlanWorkflow {
    name: String,
    steps: Vec<WorkflowStep>,
    steps_by_leg: HashMap<Uuid, Vec<String>>,
    step_to_leg: HashMap<String, Uuid>,
}

impl PlanWorkflow {
    pub fn from_plan(plan: &ExecutionPlan) -> Self {
        Self::from_plan_with_options(plan, &PlanAdapterOptions::default())
    }

    pub fn from_plan_with_options(plan: &ExecutionPlan, options: &PlanAdapterOptions) -> Self {
        let mut steps = Vec::new();
        let mut steps_by_leg = HashMap::new();
        let mut step_to_leg = HashMap::new();

        for (idx, leg) in plan.legs.iter().enumerate() {
            let leg_short = short(&leg.id);
            let mut leg_steps = Vec::new();
            let action_value = serde_json::to_value(&leg.action).unwrap_or_default();

            let simulate_id = format!("{idx}-{leg_short}-simulate");
            steps.push(WorkflowStep {
                id: simulate_id.clone(),
                activity: simulate_activity(leg.action.chain).to_string(),
                input: serde_json::json!({
                    "plan_id": plan.id,
                    "leg_id": leg.id,
                    "position": idx,
                    "action": action_value,
                }),
                retry: options.retry.clone(),
                timeout_ms: options.timeout_ms,
                requires_approval: false,
                on_failure: FailurePolicy::Halt,
            });
            leg_steps.push(simulate_id.clone());
            step_to_leg.insert(simulate_id, leg.id);

            if options.require_approval {
                let approve_id = format!("{idx}-{leg_short}-approve");
                steps.push(WorkflowStep {
                    id: approve_id.clone(),
                    activity: "approve".to_string(),
                    input: serde_json::json!({
                        "plan_id": plan.id,
                        "leg_id": leg.id,
                        "position": idx,
                    }),
                    retry: RetryPolicy::default(),
                    timeout_ms: options.approval_timeout_ms,
                    requires_approval: true,
                    on_failure: FailurePolicy::Halt,
                });
                leg_steps.push(approve_id.clone());
                step_to_leg.insert(approve_id, leg.id);
            }

            let settle_id = format!("{idx}-{leg_short}-settle");
            steps.push(WorkflowStep {
                id: settle_id.clone(),
                activity: "settle".to_string(),
                input: serde_json::json!({
                    "plan_id": plan.id,
                    "leg_id": leg.id,
                    "position": idx,
                    "action": action_value,
                }),
                retry: options.retry.clone(),
                timeout_ms: options.timeout_ms,
                requires_approval: false,
                on_failure: FailurePolicy::Halt,
            });
            leg_steps.push(settle_id.clone());
            step_to_leg.insert(settle_id, leg.id);

            steps_by_leg.insert(leg.id, leg_steps);
        }

        Self {
            name: format!("execution-plan-{}", short(&plan.id)),
            steps,
            steps_by_leg,
            step_to_leg,
        }
    }

    /// Step ids for a leg, in execution order (`simulate`, `approve?`, `settle`).
    pub fn steps_for_leg(&self, leg_id: &Uuid) -> Option<&[String]> {
        self.steps_by_leg.get(leg_id).map(|v| v.as_slice())
    }

    /// Step id to leg id mapping used to reconcile plan state.
    pub fn step_to_leg(&self) -> &HashMap<String, Uuid> {
        &self.step_to_leg
    }
}

#[async_trait]
impl WorkflowDefinition for PlanWorkflow {
    fn name(&self) -> &str {
        &self.name
    }

    fn steps(&self) -> Vec<WorkflowStep> {
        self.steps.clone()
    }
}

/// A [`WorkflowDefinition`] that runs compensating actions to unwind a partially
/// completed plan. Legs are reversed so the most recent is undone first.
#[derive(Debug, Clone)]
pub struct CompensationWorkflow {
    name: String,
    steps: Vec<WorkflowStep>,
    step_to_leg: HashMap<String, Uuid>,
}

impl CompensationWorkflow {
    pub fn from_plan(plan: &ExecutionPlan) -> Self {
        Self::from_plan_with_options(plan, &PlanAdapterOptions::default())
    }

    pub fn from_plan_with_options(plan: &ExecutionPlan, options: &PlanAdapterOptions) -> Self {
        let mut steps = Vec::new();
        let mut step_to_leg = HashMap::new();

        // Legs already marked Compensating by the AtomicityManager, undone
        // newest-first.
        let mut legs: Vec<&Leg> = plan
            .legs
            .iter()
            .filter(|l| l.status == LegStatus::Compensating && l.compensating_action.is_some())
            .collect();
        legs.reverse();

        for (idx, leg) in legs.into_iter().enumerate() {
            let step_id = format!("comp-{idx}-{}", short(&leg.id));
            let comp = leg.compensating_action.as_ref().expect("filtered above");
            steps.push(WorkflowStep {
                id: step_id.clone(),
                activity: "compensate".to_string(),
                input: serde_json::json!({
                    "plan_id": plan.id,
                    "leg_id": leg.id,
                    "description": comp.description,
                    "chain": comp.chain,
                    "action_type": comp.action_type,
                    "parameters": comp.parameters,
                }),
                retry: options.retry.clone(),
                timeout_ms: options.timeout_ms,
                requires_approval: false,
                on_failure: FailurePolicy::Halt,
            });
            step_to_leg.insert(step_id, leg.id);
        }

        Self {
            name: format!("compensate-{}", short(&plan.id)),
            steps,
            step_to_leg,
        }
    }

    /// Step id to leg id mapping used to reconcile compensation progress.
    pub fn step_to_leg(&self) -> &HashMap<String, Uuid> {
        &self.step_to_leg
    }
}

#[async_trait]
impl WorkflowDefinition for CompensationWorkflow {
    fn name(&self) -> &str {
        &self.name
    }

    fn steps(&self) -> Vec<WorkflowStep> {
        self.steps.clone()
    }
}

/// Writes the terminal outcome of a finished or crashed workflow back onto the
/// plan. A leg is `Completed` when its settle step completed and `Failed` when
/// any of its steps failed. Returns the number of legs whose status changed.
pub fn sync_plan_from_workflow(
    plan: &mut ExecutionPlan,
    adapter: &PlanWorkflow,
    state: &WorkflowState,
) -> usize {
    let mut updated = 0;

    for leg in plan.legs.iter_mut() {
        // Do not clobber compensation bookkeeping once it has begun.
        if matches!(
            leg.status,
            LegStatus::Compensating | LegStatus::Compensated | LegStatus::Skipped
        ) {
            continue;
        }
        let Some(step_ids) = adapter.steps_for_leg(&leg.id) else {
            continue;
        };
        let step_states: Vec<&crate::state::StepState> = step_ids
            .iter()
            .filter_map(|sid| state.step_states.iter().find(|ss| &ss.step_id == sid))
            .collect();
        if step_states.is_empty() {
            continue;
        }

        // Any failed step fails the leg.
        if let Some(err) = step_states.iter().find_map(|ss| match &ss.status {
            StepStatus::Failed(e) => Some(e.clone()),
            _ => None,
        }) {
            if leg.status != LegStatus::Failed {
                updated += 1;
            }
            leg.status = LegStatus::Failed;
            leg.error = Some(err);
            continue;
        }

        // The last step in a leg group is the settle step; completing it
        // completes the leg.
        let settle_id = step_ids.last().expect("leg has a settle step");
        if let Some(settle) = step_states.iter().find(|ss| &ss.step_id == settle_id)
            && matches!(settle.status, StepStatus::Completed)
            && leg.status != LegStatus::Completed
        {
            leg.status = LegStatus::Completed;
            leg.result = settle.output.clone();
            updated += 1;
        }
    }

    if matches!(
        plan.status,
        PlanStatus::Draft | PlanStatus::Validated | PlanStatus::Executing
    ) {
        plan.status = if plan.has_failed() {
            PlanStatus::Failed
        } else if plan.is_complete() {
            PlanStatus::Completed
        } else {
            PlanStatus::Executing
        };
    }

    updated
}

/// Reconciles a compensation workflow's outcomes onto the plan and manager.
///
/// Completed `compensate` steps mark the leg compensated; failed ones mark the
/// compensating action as needing manual intervention.
pub fn sync_compensation_from_workflow(
    plan: &mut ExecutionPlan,
    manager: &mut AtomicityManager,
    step_to_leg: &HashMap<String, Uuid>,
    state: &WorkflowState,
) {
    for ss in &state.step_states {
        let Some(leg_id) = step_to_leg.get(&ss.step_id) else {
            continue;
        };
        match &ss.status {
            StepStatus::Completed => manager.mark_compensated(plan, *leg_id),
            StepStatus::Failed(_) => manager.mark_compensation_failed(plan, *leg_id),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bastion_core::execution::plan::{Action, ActionType, CompensatingAction, Intent, Leg};
    use bastion_core::transaction::{Address, AgentId};
    use serde_json::json;

    fn action(chain: Chain, at: ActionType) -> Action {
        Action {
            action_type: at,
            chain,
            protocol: "test".to_string(),
            token: Some("USDC".to_string()),
            amount: Some(1000),
            destination: Some(Address::new("0xabc")),
            metadata: json!({}),
        }
    }

    fn intent3() -> Intent {
        Intent {
            description: "swap bridge stake".to_string(),
            agent_id: AgentId::new("agent-001"),
            source_chain: Chain::Ethereum,
            actions: vec![
                action(Chain::Ethereum, ActionType::Swap),
                action(Chain::Base, ActionType::Bridge),
                action(Chain::Base, ActionType::Stake),
            ],
            constraints: Vec::new(),
        }
    }

    fn plan3() -> ExecutionPlan {
        let mut p = ExecutionPlan::new(intent3());
        let mut prev: Option<Uuid> = None;
        for a in p.intent.actions.clone() {
            let id = Uuid::new_v4();
            let depends: Vec<Uuid> = prev.iter().copied().collect();
            p.legs.push(Leg {
                id,
                action: a,
                depends_on: depends,
                status: if prev.is_none() {
                    LegStatus::Ready
                } else {
                    LegStatus::Pending
                },
                compensating_action: Some(CompensatingAction {
                    description: "reverse".into(),
                    chain: Chain::Ethereum,
                    action_type: ActionType::Swap,
                    parameters: json!({}),
                }),
                result: None,
                error: None,
            });
            prev = Some(id);
        }
        p
    }

    #[test]
    fn activity_selected_per_chain() {
        let intent = Intent {
            actions: vec![
                action(Chain::Ethereum, ActionType::Swap),
                action(Chain::Base, ActionType::Bridge),
                action(Chain::Solana, ActionType::Transfer),
            ],
            ..intent3()
        };
        let mut plan = ExecutionPlan::new(intent);
        for a in plan.intent.actions.clone() {
            plan.legs.push(Leg {
                id: Uuid::new_v4(),
                action: a,
                depends_on: vec![],
                status: LegStatus::Ready,
                compensating_action: None,
                result: None,
                error: None,
            });
        }

        let wf = PlanWorkflow::from_plan_with_options(
            &plan,
            &PlanAdapterOptions {
                require_approval: false,
                ..Default::default()
            },
        );
        let acts: Vec<&str> = wf.steps.iter().map(|s| s.activity.as_str()).collect();
        // 3 legs x 2 steps (no approval): evm, settle, evm, settle, evm..., etc.
        assert_eq!(
            acts,
            vec![
                "simulate_evm",
                "settle", // Ethereum swap
                "simulate_evm",
                "settle", // Base bridge
                "simulate",
                "settle", // Solana transfer
            ]
        );
    }

    #[test]
    fn approval_steps_generated_only_when_requested() {
        let plan = plan3();
        let on = PlanWorkflow::from_plan_with_options(
            &plan,
            &PlanAdapterOptions {
                require_approval: true,
                ..Default::default()
            },
        );
        let off = PlanWorkflow::from_plan_with_options(
            &plan,
            &PlanAdapterOptions {
                require_approval: false,
                ..Default::default()
            },
        );

        assert_eq!(on.steps.len(), 9); // 3 legs x 3 steps
        assert_eq!(off.steps.len(), 6); // 3 legs x 2 steps

        let approvals_on: Vec<&str> = on
            .steps
            .iter()
            .filter(|s| s.requires_approval)
            .map(|s| s.activity.as_str())
            .collect();
        assert_eq!(approvals_on, vec!["approve", "approve", "approve"]);
        assert!(off.steps.iter().all(|s| !s.requires_approval));
    }

    #[test]
    fn compensation_workflow_reverses_completed_legs() {
        let mut plan = plan3();
        plan.legs[0].status = LegStatus::Completed;
        plan.legs[1].status = LegStatus::Completed;
        plan.legs[2].status = LegStatus::Failed;

        let mut manager = AtomicityManager::new();
        assert!(manager.should_compensate(&plan));
        let to_comp = manager.begin_compensation(&mut plan);
        assert_eq!(to_comp, vec![plan.legs[1].id, plan.legs[0].id]);

        let comp = CompensationWorkflow::from_plan(&plan);
        assert_eq!(comp.steps.len(), 2);
        assert!(comp.steps[0].activity == "compensate");
        assert!(comp.steps[0].id.contains(&short(&plan.legs[1].id)));
        assert!(comp.steps[1].id.contains(&short(&plan.legs[0].id)));
    }

    #[test]
    fn sync_marks_failed_leg_and_completes_prior_legs() {
        let mut plan = plan3();
        let adapter = PlanWorkflow::from_plan_with_options(
            &plan,
            &PlanAdapterOptions {
                require_approval: false,
                ..Default::default()
            },
        );

        let mut state = WorkflowState::new("t", Some("agent".into()));
        let steps0 = adapter.steps_for_leg(&plan.legs[0].id).unwrap().to_vec();
        let steps1 = adapter.steps_for_leg(&plan.legs[1].id).unwrap().to_vec();

        let mut simulate = crate::state::StepState::new(steps0[0].clone(), json!({}));
        simulate.status = StepStatus::Completed;
        let mut settle0 = crate::state::StepState::new(steps0[1].clone(), json!({}));
        settle0.status = StepStatus::Completed;
        let mut settle1 = crate::state::StepState::new(steps1.last().unwrap().clone(), json!({}));
        settle1.status = StepStatus::Failed("reverted on Base".into());
        state.step_states = vec![simulate, settle0, settle1];

        let updated = sync_plan_from_workflow(&mut plan, &adapter, &state);
        assert!(updated >= 2);
        assert_eq!(plan.legs[0].status, LegStatus::Completed);
        assert_eq!(plan.legs[1].status, LegStatus::Failed);
        assert_eq!(plan.legs[1].error.as_deref(), Some("reverted on Base"));
        assert_eq!(plan.status, PlanStatus::Failed);
    }
}
