use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkflowError {
    #[error("workflow not found: {0}")]
    NotFound(String),

    #[error("workflow already exists: {0}")]
    AlreadyExists(String),

    #[error("invalid state transition: {workflow_id} from {from:?} to {to:?}")]
    InvalidTransition {
        workflow_id: String,
        from: crate::state::WorkflowStatus,
        to: crate::state::WorkflowStatus,
    },

    #[error("step not found: {0}")]
    StepNotFound(String),

    #[error("activity error: {0}")]
    ActivityError(String),

    #[error("activity timeout: {step_id} after {timeout_ms}ms")]
    ActivityTimeout { step_id: String, timeout_ms: u64 },

    #[error("retry exhausted: {step_id} failed {attempts} times")]
    RetryExhausted { step_id: String, attempts: u32 },

    #[error("signal rejected: {reason}")]
    SignalRejected { reason: String },

    #[error("workflow cancelled")]
    Cancelled,

    #[error("storage error: {0}")]
    Storage(String),

    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("validation error: {0}")]
    Validation(String),
}

impl From<serde_json::Error> for WorkflowError {
    fn from(e: serde_json::Error) -> Self {
        Self::Serialization(e.to_string())
    }
}

impl From<sled::Error> for WorkflowError {
    fn from(e: sled::Error) -> Self {
        Self::Storage(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, WorkflowError>;
