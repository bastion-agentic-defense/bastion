use async_trait::async_trait;
use crate::{Activity, ActivityContext, WorkflowError};

pub struct SleepActivity;

#[async_trait]
impl Activity for SleepActivity {
    fn name(&self) -> &str { "sleep" }

    async fn execute(
        &self,
        input: serde_json::Value,
        _ctx: ActivityContext,
    ) -> Result<serde_json::Value, WorkflowError> {
        let ms = input.get("ms").and_then(|v| v.as_u64()).unwrap_or(1000);
        tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
        Ok(serde_json::json!({ "slept_ms": ms }))
    }
}
