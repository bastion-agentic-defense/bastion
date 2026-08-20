use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

/// Chain-agnostic simulation outcome shared by every simulator (EVM today,
/// formerly Solana). `error`, `units_consumed`, and `balance_changes` are all
/// chain-neutral, which is why `policy.rs`'s `SimulationCheck` impls and
/// `simulation_evm.rs` depend on this struct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResult {
    pub logs: Vec<String>,
    pub units_consumed: Option<u64>,
    pub return_data: Option<ReturnData>,
    pub error: Option<serde_json::Value>,
    #[serde(default)]
    pub balance_changes: HashMap<String, i64>,
    #[serde(default)]
    pub simulation_hash: Option<[u8; 32]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnData {
    pub data: String,
    pub encoding: String,
    pub program_id: String,
}

/// Deterministic content hash of a simulation outcome, used to anchor audit
/// records. Chain-agnostic: it hashes only the logs and error payload.
pub fn compute_simulation_hash(logs: &[String], error: &Option<serde_json::Value>) -> [u8; 32] {
    let mut hasher = Sha256::new();
    for log in logs {
        hasher.update(log.as_bytes());
        hasher.update(b"\0");
    }
    if let Some(err) = error {
        hasher.update(err.to_string().as_bytes());
    }
    let hash = hasher.finalize();
    let mut result = [0u8; 32];
    result.copy_from_slice(&hash);
    result
}
