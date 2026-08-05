use async_trait::async_trait;
use crate::{Activity, ActivityContext, WorkflowError};

pub struct SettleActivity;

#[async_trait]
impl Activity for SettleActivity {
    fn name(&self) -> &str { "settle" }

    async fn execute(
        &self,
        input: serde_json::Value,
        _ctx: ActivityContext,
    ) -> Result<serde_json::Value, WorkflowError> {
        Ok(serde_json::json!({
            "status": "settled",
            "audit_id": format!("audit_{}", uuid::Uuid::new_v4()),
            "input": input
        }))
    }
}
