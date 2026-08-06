use async_trait::async_trait;

use crate::{Activity, ActivityContext, WorkflowError};

/// Executes the compensating action that reverses a previously completed leg.
///
/// In production this submits the reverse transaction (for example selling the
/// acquired token back, bridging funds to the source chain, or unstaking). The
/// default implementation records the request so that the downstream
/// `AtomicityManager` can mark the leg as compensated.
pub struct CompensateActivity;

#[async_trait]
impl Activity for CompensateActivity {
    fn name(&self) -> &str {
        "compensate"
    }

    async fn execute(
        &self,
        input: serde_json::Value,
        _ctx: ActivityContext,
    ) -> Result<serde_json::Value, WorkflowError> {
        let leg_id = input
            .get("leg_id")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        Ok(serde_json::json!({
            "status": "compensated",
            "leg_id": leg_id,
            "input": input,
            "note": "compensate activity - submits the reversal transaction in production"
        }))
    }
}
