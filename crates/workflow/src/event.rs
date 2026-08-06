use serde::{Deserialize, Serialize};

use super::error::Result;
use crate::state::{StepStatus, WorkflowState, WorkflowStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WorkflowEvent {
    WorkflowStarted {
        id: String,
        definition: String,
        timestamp: u64,
    },
    StepStarted {
        id: String,
        step: String,
        attempt: u32,
        timestamp: u64,
    },
    StepCompleted {
        id: String,
        step: String,
        output: serde_json::Value,
        timestamp: u64,
    },
    StepFailed {
        id: String,
        step: String,
        error: String,
        attempt: u32,
        timestamp: u64,
    },
    StepRetrying {
        id: String,
        step: String,
        attempt: u32,
        backoff_ms: u64,
        timestamp: u64,
    },
    WorkflowPaused {
        id: String,
        step: String,
        reason: String,
        timestamp: u64,
    },
    WorkflowResumed {
        id: String,
        by: String,
        timestamp: u64,
    },
    WorkflowCompleted {
        id: String,
        timestamp: u64,
    },
    WorkflowFailed {
        id: String,
        error: String,
        timestamp: u64,
    },
    WorkflowCancelled {
        id: String,
        timestamp: u64,
    },
}

impl WorkflowEvent {
    pub fn now(ev: Self) -> Self {
        ev
    }

    pub fn ts(&self) -> u64 {
        match self {
            Self::WorkflowStarted { timestamp, .. }
            | Self::StepStarted { timestamp, .. }
            | Self::StepCompleted { timestamp, .. }
            | Self::StepFailed { timestamp, .. }
            | Self::StepRetrying { timestamp, .. }
            | Self::WorkflowPaused { timestamp, .. }
            | Self::WorkflowResumed { timestamp, .. }
            | Self::WorkflowCompleted { timestamp, .. }
            | Self::WorkflowFailed { timestamp, .. }
            | Self::WorkflowCancelled { timestamp, .. } => *timestamp,
        }
    }
}

pub fn timestamp_now() -> u64 {
    chrono::Utc::now().timestamp() as u64
}

pub fn events(history: &[WorkflowEvent], state: &mut WorkflowState) -> Result<()> {
    for ev in history {
        apply(ev, state)?;
    }
    Ok(())
}

fn apply(ev: &WorkflowEvent, state: &mut WorkflowState) -> Result<()> {
    match ev {
        WorkflowEvent::WorkflowStarted { id, definition, .. } => {
            state.id = id.clone();
            state.definition = definition.clone();
            state.status = WorkflowStatus::Running;
            state.current_step = 0;
        }
        WorkflowEvent::StepStarted { step, attempt, .. } => {
            if let Some(ss) = state.step_states.iter_mut().find(|s| &s.step_id == step) {
                ss.status = StepStatus::Running;
                ss.attempt = *attempt;
                ss.started_at = Some(ev.ts());
            }
        }
        WorkflowEvent::StepCompleted { step, output, .. } => {
            if let Some(ss) = state.step_states.iter_mut().find(|s| &s.step_id == step) {
                ss.status = StepStatus::Completed;
                ss.output = Some(output.clone());
                ss.completed_at = Some(ev.ts());
            }
            state.current_step = state.current_step.max(
                state
                    .step_states
                    .iter()
                    .position(|s| &s.step_id == step)
                    .unwrap_or(0)
                    + 1,
            );
        }
        WorkflowEvent::StepFailed {
            step,
            error,
            attempt,
            ..
        } => {
            if let Some(ss) = state.step_states.iter_mut().find(|s| &s.step_id == step) {
                ss.status = StepStatus::Failed(error.clone());
                ss.attempt = *attempt;
                ss.error = Some(error.clone());
            }
        }
        WorkflowEvent::WorkflowPaused { reason: _, .. } => {
            state.status = WorkflowStatus::Paused;
        }
        WorkflowEvent::WorkflowResumed { .. } => {
            state.status = WorkflowStatus::Running;
        }
        WorkflowEvent::WorkflowCompleted { .. } => {
            state.status = WorkflowStatus::Completed;
        }
        WorkflowEvent::WorkflowFailed { error, .. } => {
            state.status = WorkflowStatus::Failed(error.clone());
        }
        WorkflowEvent::WorkflowCancelled { .. } => {
            state.status = WorkflowStatus::Cancelled;
        }
        _ => {}
    }
    state.mark_updated();
    Ok(())
}
