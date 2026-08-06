use crate::{Activity, ActivityContext, WorkflowError};
use async_trait::async_trait;

pub struct ApproveActivity;

#[async_trait]
impl Activity for ApproveActivity {
    fn name(&self) -> &str {
        "approve"
    }

    async fn execute(
        &self,
        input: serde_json::Value,
        _ctx: ActivityContext,
    ) -> Result<serde_json::Value, WorkflowError> {
        Ok(serde_json::json!({
            "status": "approved",
            "input": input,
            "note": "approve activity - resolved by human override signal in production"
        }))
    }
}
