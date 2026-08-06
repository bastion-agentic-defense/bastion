use crate::{Activity, ActivityContext, WorkflowError};
use async_trait::async_trait;

pub struct FetchSecretActivity;

#[async_trait]
impl Activity for FetchSecretActivity {
    fn name(&self) -> &str {
        "fetch_secret"
    }

    async fn execute(
        &self,
        input: serde_json::Value,
        _ctx: ActivityContext,
    ) -> Result<serde_json::Value, WorkflowError> {
        let key = input
            .get("key")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        Ok(serde_json::json!({
            "key": key,
            "note": "fetch_secret stub - Vault integration planned for Phase 3"
        }))
    }
}
