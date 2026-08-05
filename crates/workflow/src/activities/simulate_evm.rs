use async_trait::async_trait;
use crate::{Activity, ActivityContext, WorkflowError};

pub struct SimulateEvmActivity;

#[async_trait]
impl Activity for SimulateEvmActivity {
    fn name(&self) -> &str { "simulate_evm" }

    async fn execute(
        &self,
        input: serde_json::Value,
        _ctx: ActivityContext,
    ) -> Result<serde_json::Value, WorkflowError> {
        Ok(serde_json::json!({
            "status": "simulated_evm",
            "input": input,
            "note": "simulate_evm activity - connect to sidecar POST /api/v2/simulate-evm in production"
        }))
    }
}
