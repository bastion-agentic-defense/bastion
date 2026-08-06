use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowStatus {
    Running,
    Paused,
    Completed,
    Failed(String),
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum StepStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
    Paused,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowState {
    #[serde(default = "default_id")]
    pub id: String,
    pub definition: String,
    pub status: WorkflowStatus,
    pub current_step: usize,
    pub step_states: Vec<StepState>,
    #[serde(default = "default_now")]
    pub created_at: u64,
    #[serde(default = "default_now")]
    pub updated_at: u64,
    pub agent_id: Option<String>,
    pub tags: std::collections::HashMap<String, String>,
}

fn default_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

fn default_now() -> u64 {
    chrono::Utc::now().timestamp() as u64
}

impl WorkflowState {
    pub fn new(definition: impl Into<String>, agent_id: Option<String>) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp() as u64;
        Self {
            id,
            definition: definition.into(),
            status: WorkflowStatus::Running,
            current_step: 0,
            step_states: Vec::new(),
            created_at: now,
            updated_at: now,
            agent_id,
            tags: std::collections::HashMap::new(),
        }
    }

    pub fn mark_updated(&mut self) {
        self.updated_at = chrono::Utc::now().timestamp() as u64;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepState {
    pub step_id: String,
    pub status: StepStatus,
    pub input: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub attempt: u32,
    pub started_at: Option<u64>,
    pub completed_at: Option<u64>,
    pub error: Option<String>,
}

impl StepState {
    pub fn new(step_id: impl Into<String>, input: serde_json::Value) -> Self {
        Self {
            step_id: step_id.into(),
            status: StepStatus::Pending,
            input,
            output: None,
            attempt: 0,
            started_at: None,
            completed_at: None,
            error: None,
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(
            self.status,
            StepStatus::Completed | StepStatus::Failed(_) | StepStatus::Skipped
        )
    }
}
