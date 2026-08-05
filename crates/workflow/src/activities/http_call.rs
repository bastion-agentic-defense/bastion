use async_trait::async_trait;
use crate::{Activity, ActivityContext, WorkflowError};

pub struct HttpCallActivity;

#[async_trait]
impl Activity for HttpCallActivity {
    fn name(&self) -> &str { "http_call" }

    async fn execute(
        &self,
        input: serde_json::Value,
        _ctx: ActivityContext,
    ) -> Result<serde_json::Value, WorkflowError> {
        Ok(serde_json::json!({
            "status": "ok",
            "status_code": 200,
            "input": input,
            "note": "http_call activity - routes through Web2 firewall proxy in production"
        }))
    }
}
