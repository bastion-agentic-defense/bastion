use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

pub struct ActivityContext {
    pub workflow_id: String,
    pub step_id: String,
    pub attempt: u32,
    pub agent_id: Option<String>,
}

#[async_trait]
pub trait Activity: Send + Sync {
    fn name(&self) -> &str;

    async fn execute(
        &self,
        input: serde_json::Value,
        ctx: ActivityContext,
    ) -> Result<serde_json::Value, crate::error::WorkflowError>;
}

pub struct ActivityRegistry {
    activities: HashMap<String, Arc<dyn Activity>>,
}

impl ActivityRegistry {
    pub fn new() -> Self {
        Self { activities: HashMap::new() }
    }

    pub fn register<A: Activity + 'static>(&mut self, activity: A) {
        let name = activity.name().to_string();
        self.activities.insert(name, Arc::new(activity));
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn Activity>> {
        self.activities.get(name).cloned()
    }

    pub fn list(&self) -> Vec<String> {
        self.activities.keys().cloned().collect()
    }
}

impl Default for ActivityRegistry {
    fn default() -> Self {
        Self::new()
    }
}
