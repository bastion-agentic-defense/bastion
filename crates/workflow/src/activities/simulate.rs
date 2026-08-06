use crate::{Activity, ActivityContext, WorkflowError};
use async_trait::async_trait;

pub struct SimulateActivity;

#[async_trait]
impl Activity for SimulateActivity {
    fn name(&self) -> &str {
        "simulate"
    }

    async fn execute(
        &self,
        input: serde_json::Value,
        _ctx: ActivityContext,
    ) -> Result<serde_json::Value, WorkflowError> {
        Ok(serde_json::json!({
            "status": "simulated",
            "input": input,
            "note": "simulate activity - connect to sidecar POST /simulate in production"
        }))
    }
}
