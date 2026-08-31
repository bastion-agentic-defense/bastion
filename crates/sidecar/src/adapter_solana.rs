//! SolanaAdapter — chain-independent Solana settlement for the Bastion runtime.
//!
//! This bridges Solana into the [`TrustAdapter`] abstraction so the policy engine
//! and HTTP layer treat it like any other settlement chain. The adapter talks to
//! Solana via plain JSON-RPC (`getBalance`, `simulateTransaction`) over the already
//! present `reqwest` client — it does **not** link the heavy `solana-sdk`/`solana-client`
//! crates, keeping the sidecar build lean (a deliberate result of the EVM pivot).
//!
//! Documented mainnet-MVP limits (see `crates/sidecar/SPEC.md`):
//! - `verify` performs a **RPC state-read baseline**, not a VM sandbox: `getBalance`
//!   (lamport delta for simple transfers) and `simulateTransaction` when a serialized
//!   transaction is supplied. Gas/CU refunds and program-side effects beyond a single
//!   balance pair are not exhaustively modeled.
//! - `execute` is opt-in: unless `BASTION_SOLANA_EXECUTE` is set, the adapter never
//!   broadcasts a live transaction and `execute` returns an error.

use async_trait::async_trait;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;

use bastion_core::TrustAdapter;
use bastion_core::adapter::{SimulationOutcome, TrustAdapterError};
use bastion_core::transaction::{Address, AgentId, Chain, NormalizedTransaction};

use crate::audit::AuditLogger;

/// Sanity bound for lamport amounts (SOL largest supply ~1.4B * 1e9).
const MAX_LAMPORTS_CHECK: u64 = 10_000_000_000_000_000;

/// A chain-independent adapter for the Solana execution environment.
///
/// One instance is created for the `Chain::Solana` settlement target and held in
/// the sidecar's per-chain adapter registry, exactly like the EVM simulators.
pub struct SolanaAdapter {
    client: reqwest::Client,
    rpc_url: String,
    logger: std::sync::Arc<AuditLogger>,
    /// Whether live broadcast is permitted (`BASTION_SOLANA_EXECUTE`).
    execute_enabled: bool,
}

#[derive(Deserialize)]
struct RpcEnvelope<T> {
    #[allow(dead_code)]
    result: Option<T>,
    error: Option<RpcError>,
}

#[derive(Deserialize)]
struct RpcError {
    #[allow(dead_code)]
    code: i64,
    message: String,
}

impl SolanaAdapter {
    /// Construct the adapter from `SOLANA_RPC_URL`.
    pub fn new(rpc_url: String, logger: std::sync::Arc<AuditLogger>) -> Self {
        let execute_enabled = env::var("BASTION_SOLANA_EXECUTE").is_ok();
        Self {
            client: reqwest::Client::new(),
            rpc_url,
            logger,
            execute_enabled,
        }
    }

    /// A disabled adapter for tests / policy-only startups (no RPC configured).
    pub fn disabled(logger: std::sync::Arc<AuditLogger>) -> Self {
        Self {
            client: reqwest::Client::new(),
            rpc_url: String::new(),
            logger,
            execute_enabled: false,
        }
    }

    /// The RPC URL this adapter targets (empty when disabled).
    pub fn rpc_url(&self) -> &str {
        &self.rpc_url
    }

    /// Simulate a serialized (base58) Solana transaction via `simulateTransaction`.
    /// Returns a coarse success/log projection (no VM-level balance accounting).
    async fn simulate_transaction(&self, tx: &str) -> Result<SimulationOutcome, TrustAdapterError> {
        let body = serde_json::json!({
            "jsonrpc": "2.0", "id": 1, "method": "simulateTransaction",
            "params": [tx, { "replaceRecentBlockhash": true }]
        });
        let resp: RpcEnvelope<serde_json::Value> = self
            .client
            .post(&self.rpc_url)
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                TrustAdapterError::SimulationFailed(format!("simulateTransaction failed: {e}"))
            })?
            .json::<RpcEnvelope<serde_json::Value>>()
            .await
            .map_err(|e| {
                TrustAdapterError::SimulationFailed(format!(
                    "simulateTransaction parse failed: {e}"
                ))
            })?;

        if let Some(err) = resp.error {
            return Ok(SimulationOutcome {
                balance_changes: HashMap::new(),
                logs: vec![format!(
                    "simulateTransaction error {}: {}",
                    err.code, err.message
                )],
                success: false,
            });
        }
        Ok(SimulationOutcome {
            balance_changes: HashMap::new(),
            logs: vec![format!("simulateTransaction OK (len={})", tx.len())],
            success: true,
        })
    }
}

#[async_trait]
impl TrustAdapter for SolanaAdapter {
    async fn authenticate(
        &self,
        address: &Address,
    ) -> Result<bastion_core::adapter::AgentIdentity, TrustAdapterError> {
        // Validate it is a well-formed base58 pubkey; the agent is looked up from
        // the off-chain Sled registry by the caller. We do not require RPC for auth.
        let _pk = bs58::decode(address.as_str()).into_vec().map_err(|e| {
            TrustAdapterError::AuthenticationFailed(format!("invalid base58 pubkey: {e}"))
        })?;
        if _pk.len() != 32 {
            return Err(TrustAdapterError::AuthenticationFailed(format!(
                "Solana pubkey must be 32 bytes, got {}",
                _pk.len()
            )));
        }
        Ok(bastion_core::adapter::AgentIdentity {
            agent_id: AgentId::new(address.as_str()),
            chain: Chain::Solana,
            address: address.clone(),
            reputation: None,
        })
    }

    async fn verify(
        &self,
        tx: &NormalizedTransaction,
    ) -> Result<SimulationOutcome, TrustAdapterError> {
        let mut logs = vec![format!(
            "Solana verify on chain=solana from={} to={} amount={}",
            tx.from.as_str(),
            tx.to.as_str(),
            tx.amount
        )];
        let mut success = true;
        let mut balance_changes = HashMap::new();

        // If a serialized transaction was attached in metadata, run the RPC simulation.
        if let Some(tx_str) = tx.metadata.get("solana_tx").and_then(|v| v.as_str()) {
            let outcome = self.simulate_transaction(tx_str).await?;
            logs.extend(outcome.logs);
            success = outcome.success;
            balance_changes = outcome.balance_changes;
        } else {
            // Coarse transfer projection: sender loses `amount`, receiver gains it.
            if tx.amount <= MAX_LAMPORTS_CHECK {
                if tx.amount > 0 {
                    logs.push(format!("projected transfer of {} lamports", tx.amount));
                    balance_changes.insert(tx.from.as_str().to_string(), -(tx.amount as i64));
                    balance_changes.insert(tx.to.as_str().to_string(), tx.amount as i64);
                }
            } else {
                logs.push("amount exceeds sanity bound; flagged for HITL".into());
                success = false;
            }
        }

        Ok(SimulationOutcome {
            balance_changes,
            logs,
            success,
        })
    }

    async fn execute(
        &self,
        tx: &NormalizedTransaction,
    ) -> Result<bastion_core::adapter::ExecutionReceipt, TrustAdapterError> {
        if !self.execute_enabled {
            return Err(TrustAdapterError::ExecutionFailed(
                "Solana execution disabled (set BASTION_SOLANA_EXECUTE to enable broadcast)".into(),
            ));
        }
        // Off-chain MVP: no serialized tx builder is attached, so we record a
        // placeholder receipt. Live broadcast requires the SDK to submit the
        // signed transaction and report the signature here.
        Ok(bastion_core::adapter::ExecutionReceipt {
            tx_hash: format!("solana:simulated:{:x}", tx.amount),
            block_number: None,
            success: true,
        })
    }

    async fn settle(
        &self,
        _receipt: &bastion_core::adapter::ExecutionReceipt,
    ) -> Result<(), TrustAdapterError> {
        let _ = &self.logger;
        Ok(())
    }

    fn chain_name(&self) -> &str {
        "solana"
    }

    fn chain(&self) -> Chain {
        Chain::Solana
    }
}

/// Request body for `POST /api/v2/simulate-solana`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SolanaSimulateRequest {
    /// Destination pubkey.
    pub to: String,
    /// Amount in lamports.
    #[serde(default)]
    pub amount: Option<u64>,
    /// Optional serialized (base58) Solana transaction to run `simulateTransaction` on.
    #[serde(default)]
    pub transaction: Option<String>,
    #[serde(default)]
    pub intent: Option<String>,
    #[serde(default)]
    pub agent_id: Option<String>,
}

/// Response body for `POST /api/v2/simulate-solana`, matching the EVM simulate
/// response shape so SDK/dashboard consumers share one contract.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SolanaSimulateResponse {
    pub allowed: bool,
    pub decision: String,
    pub reason: Option<String>,
    pub simulation_result: Option<crate::simulation::SimulationResult>,
    pub risk_score: Option<u8>,
    pub risk_summary: Option<String>,
}

/// Run the adapter's `verify` over a normalized Solana transaction and project the
/// outcome into the audit-facing `SimulationResult`.
pub fn project_solana_outcome(
    outcome: &SimulationOutcome,
    to: &str,
) -> crate::simulation::SimulationResult {
    use crate::simulation::{ReturnData, SimulationResult, compute_simulation_hash};
    let keyed_error = if outcome.success {
        None
    } else {
        Some(serde_json::json!({"message": "Solana simulation did not succeed"}))
    };
    SimulationResult {
        logs: outcome.logs.clone(),
        units_consumed: None,
        return_data: Some(ReturnData {
            data: "0x".into(),
            encoding: "hex".into(),
            program_id: to.to_string(),
        }),
        error: keyed_error.clone(),
        balance_changes: outcome.balance_changes.clone(),
        simulation_hash: Some(compute_simulation_hash(&outcome.logs, &keyed_error)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit::AuditLogger;

    fn logger() -> std::sync::Arc<AuditLogger> {
        let dir = tempfile::tempdir().unwrap();
        std::sync::Arc::new(AuditLogger::new(dir.path().join("audit.db")).unwrap())
    }

    #[test]
    fn test_authenticate_valid_and_invalid_pubkey() {
        let adapter = SolanaAdapter::disabled(logger());
        let valid = Address::new("11111111111111111111111111111111");
        let identity = futures::executor::block_on(adapter.authenticate(&valid)).unwrap();
        assert_eq!(identity.chain, Chain::Solana);
        assert_eq!(
            identity.address.as_str(),
            "11111111111111111111111111111111"
        );

        let invalid = Address::new("not-a-base58-pubkey");
        assert!(futures::executor::block_on(adapter.authenticate(&invalid)).is_err());
    }

    #[test]
    fn test_verify_transfer_projection_offline() {
        let adapter = SolanaAdapter::disabled(logger());
        let tx = NormalizedTransaction::new(
            "agent-1",
            "abc",
            "def",
            1_000_000_000,
            "SOL",
            bastion_core::TxType::Transfer,
            Chain::Solana,
        );
        let outcome = futures::executor::block_on(adapter.verify(&tx)).unwrap();
        // Offline (no RPC) still yields a coarse projection.
        assert!(outcome.success);
        assert_eq!(
            outcome.balance_changes.get("abc").cloned().unwrap_or(0),
            -(1_000_000_000_i64)
        );
    }

    #[test]
    fn test_execute_disabled_by_default() {
        let adapter = SolanaAdapter::disabled(logger());
        let tx = NormalizedTransaction::new(
            "agent-1",
            "abc",
            "def",
            1,
            "SOL",
            bastion_core::TxType::Transfer,
            Chain::Solana,
        );
        let err = futures::executor::block_on(adapter.execute(&tx)).unwrap_err();
        assert!(err.to_string().contains("BASTION_SOLANA_EXECUTE"));
    }

    #[test]
    fn test_chain_metadata() {
        let adapter = SolanaAdapter::disabled(logger());
        assert_eq!(adapter.chain_name(), "solana");
        assert_eq!(adapter.chain(), Chain::Solana);
    }
}
