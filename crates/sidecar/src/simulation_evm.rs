use anyhow::{Result, anyhow};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

use crate::simulation::{ReturnData, SimulationResult, compute_simulation_hash};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvmTxParams {
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub data: Option<String>,
    #[serde(default)]
    pub gas: Option<String>,
    #[serde(default)]
    pub gas_price: Option<String>,
    #[serde(default, rename = "maxFeePerGas")]
    pub max_fee_per_gas: Option<String>,
    #[serde(default, rename = "maxPriorityFeePerGas")]
    pub max_priority_fee_per_gas: Option<String>,
    #[serde(default)]
    pub nonce: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvmSimulateRequest {
    pub transaction: EvmTxParams,
    pub intent: Option<String>,
    pub chain: Option<String>,
    pub agent_id: Option<String>,
}

pub trait EvmSimulate: Send + Sync {
    fn simulate_evm_tx(&self, tx: &EvmTxParams) -> Result<SimulationResult>;
}

/// Canonical mapping of normalized chain name → the env var that configures its
/// RPC endpoint. Single source of truth shared by the sidecar startup (which
/// builds the simulators) and the request handler (which names the missing var
/// in its 503). Note testnet chains use non-derivable var names (e.g. "sepolia"
/// → `ETH_SEPOLIA_RPC_URL`), which is why this can't be derived from the chain.
pub const EVM_CHAIN_ENV_VARS: &[(&str, &str)] = &[
    ("ethereum", "ETH_RPC_URL"),
    ("base", "BASE_RPC_URL"),
    ("celo", "CELO_RPC_URL"),
    ("sepolia", "ETH_SEPOLIA_RPC_URL"),
    ("zksync", "ZKSYNC_RPC_URL"),
    ("zksync_sepolia", "ZKSYNC_SEPOLIA_RPC_URL"),
    ("robinhood", "ROBINHOOD_RPC_URL"),
    ("robinhood_testnet", "ROBINHOOD_TESTNET_RPC_URL"),
    ("polygon", "POLYGON_RPC_URL"),
    ("arbitrum", "ARBITRUM_RPC_URL"),
    ("monad", "MONAD_RPC_URL"),
];

/// The RPC env var name for a chain, e.g. `evm_rpc_env_var("ethereum") == "ETH_RPC_URL"`.
/// Falls back to `<CHAIN>_RPC_URL` for chains not in the canonical table.
pub fn evm_rpc_env_var(chain: &str) -> String {
    EVM_CHAIN_ENV_VARS
        .iter()
        .find(|(c, _)| *c == chain)
        .map(|(_, v)| (*v).to_string())
        .unwrap_or_else(|| format!("{}_RPC_URL", chain.to_ascii_uppercase()))
}

/// A chain-agnostic EVM transaction simulator.
///
/// The behavior is pure JSON-RPC (`eth_call` + balance diff), so a single
/// implementation serves any EVM chain - the caller supplies the RPC URL and a
/// human-readable `chain_label` (used in logs and audit records). One instance
/// is created per configured chain and selected by the request's `chain` field.
pub struct EvmSimulator {
    client: Client,
    rpc_url: String,
    chain_label: String,
}

impl EvmSimulator {
    /// Construct the Celo simulator from `CELO_RPC_URL` (default `forno.celo.org`).
    /// Retained for back-compat; prefer [`EvmSimulator::for_chain`] for other chains.
    pub fn new() -> Result<Self> {
        let rpc_url =
            env::var("CELO_RPC_URL").unwrap_or_else(|_| "https://forno.celo.org".to_string());
        Ok(Self {
            client: Client::new(),
            rpc_url,
            chain_label: "celo".to_string(),
        })
    }

    /// Construct a simulator for an arbitrary chain label + RPC URL.
    pub fn for_chain(chain_label: impl Into<String>, rpc_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            rpc_url: rpc_url.into(),
            chain_label: chain_label.into(),
        }
    }

    /// Construct from an RPC URL with an unspecified chain label (defaults to "evm").
    pub fn from_rpc_url(rpc_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            rpc_url: rpc_url.into(),
            chain_label: "evm".to_string(),
        }
    }

    /// The chain this simulator targets (e.g. "ethereum", "celo", "sepolia").
    pub fn chain_label(&self) -> &str {
        &self.chain_label
    }

    fn fetch_balance(&self, address: &str) -> Result<u128> {
        let body = serde_json::json!({
            "jsonrpc": "2.0", "id": 1, "method": "eth_getBalance",
            "params": [address, "latest"]
        });
        let response = self
            .client
            .post(&self.rpc_url)
            .json(&body)
            .send()
            .map_err(|e| anyhow!("eth_getBalance request failed: {e}"))?;
        let resp: RpcResponse<String> = response
            .json()
            .map_err(|e| anyhow!("eth_getBalance parse failed: {e}"))?;
        let hex = resp
            .result
            .ok_or_else(|| anyhow!("eth_getBalance returned no result"))?;
        u128::from_str_radix(hex.trim_start_matches("0x"), 16)
            .map_err(|e| anyhow!("Failed to parse balance hex: {e}"))
    }
}

impl EvmSimulate for EvmSimulator {
    fn simulate_evm_tx(&self, tx: &EvmTxParams) -> Result<SimulationResult> {
        let pre_from = self.fetch_balance(&tx.from).unwrap_or(0);
        let pre_to = self.fetch_balance(&tx.to).unwrap_or(0);

        let call_params = serde_json::json!({
            "from": tx.from,
            "to": tx.to,
            "value": tx.value.as_deref().unwrap_or("0x0"),
            "data": tx.data.as_deref().unwrap_or("0x"),
            "gas": tx.gas.as_deref().unwrap_or("0x4C4B40"),
            "gasPrice": tx.gas_price.as_deref().unwrap_or("0x0"),
        });

        let body = serde_json::json!({
            "jsonrpc": "2.0", "id": 1, "method": "eth_call",
            "params": [call_params, "latest"]
        });

        let response = self
            .client
            .post(&self.rpc_url)
            .json(&body)
            .send()
            .map_err(|e| anyhow!("eth_call request failed: {e}"))?;
        let resp: RpcCallResponse = response
            .json()
            .map_err(|e| anyhow!("eth_call parse failed: {e}"))?;

        let mut sim_logs = Vec::new();
        let mut sim_error: Option<serde_json::Value> = None;
        let mut return_data: Option<ReturnData> = None;

        if let Some(ref data) = resp.result {
            if data.is_empty() || data == "0x" {
                sim_logs.push("eth_call returned 0x (empty)".to_string());
            } else {
                sim_logs.push(format!("eth_call succeeded, return data: {data}"));
                return_data = Some(ReturnData {
                    data: data.clone(),
                    encoding: "hex".to_string(),
                    program_id: tx.to.clone(),
                });
            }
        }

        if let Some(ref err) = resp.error {
            sim_error = Some(serde_json::json!({
                "code": err.code,
                "message": err.message.clone(),
            }));
            sim_logs.push(format!(
                "eth_call error: {} (code {})",
                err.message, err.code
            ));
        }

        let post_from = self.fetch_balance(&tx.from).unwrap_or(pre_from);
        let post_to = self.fetch_balance(&tx.to).unwrap_or(pre_to);

        let value_hex = tx.value.as_deref().unwrap_or("0x0");
        let value = u128::from_str_radix(value_hex.trim_start_matches("0x"), 16).unwrap_or(0);

        let mut balance_changes = HashMap::new();
        let from_delta = (post_from as i128) - (pre_from as i128) - (value as i128);
        if from_delta != 0 {
            balance_changes.insert(tx.from.clone(), from_delta as i64);
        }
        let to_delta = (post_to as i128) - (pre_to as i128) + (value as i128);
        if to_delta != 0 {
            balance_changes.insert(tx.to.clone(), to_delta as i64);
        }

        let sim_hash = compute_simulation_hash(&sim_logs, &sim_error);

        Ok(SimulationResult {
            logs: sim_logs,
            units_consumed: None,
            return_data,
            error: sim_error,
            balance_changes,
            simulation_hash: Some(sim_hash),
        })
    }
}

#[derive(Debug, Deserialize)]
struct RpcResponse<T> {
    result: Option<T>,
    #[allow(dead_code)]
    error: Option<RpcErrorBody>,
}

#[derive(Debug, Deserialize)]
struct RpcCallResponse {
    result: Option<String>,
    error: Option<RpcErrorBody>,
}

#[derive(Debug, Deserialize)]
struct RpcErrorBody {
    code: i64,
    message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvmSimulateResponse {
    pub allowed: bool,
    pub decision: String,
    pub reason: Option<String>,
    pub simulation_result: Option<SimulationResult>,
    pub risk_score: Option<u8>,
    pub risk_summary: Option<String>,
}

/// Map a normalized EVM chain label to its `Chain` variant.
pub fn chain_from_label(label: &str) -> bastion_core::transaction::Chain {
    match label {
        "base" => bastion_core::transaction::Chain::Base,
        "polygon" => bastion_core::transaction::Chain::Polygon,
        "arbitrum" => bastion_core::transaction::Chain::Arbitrum,
        "celo" => bastion_core::transaction::Chain::Celo,
        "zksync" | "zksync_sepolia" => bastion_core::transaction::Chain::ZkSync,
        "robinhood" | "robinhood_testnet" => bastion_core::transaction::Chain::Robinhood,
        // "ethereum", "sepolia", and any unrecognized label default to Ethereum.
        _ => bastion_core::transaction::Chain::Ethereum,
    }
}

#[async_trait::async_trait]
impl bastion_core::adapter::TrustAdapter for EvmSimulator {
    async fn authenticate(
        &self,
        address: &bastion_core::transaction::Address,
    ) -> Result<bastion_core::adapter::AgentIdentity, bastion_core::TrustAdapterError> {
        // EVM addresses are are 0x + 40 hex chars; validate structurally without RPC.
        let s = address.as_str();
        if !s.starts_with("0x") || s.len() != 42 {
            return Err(bastion_core::TrustAdapterError::AuthenticationFailed(
                format!("invalid EVM address: {s}"),
            ));
        }
        Ok(bastion_core::adapter::AgentIdentity {
            agent_id: bastion_core::transaction::AgentId::new(s),
            chain: chain_from_label(&self.chain_label),
            address: address.clone(),
            reputation: None,
        })
    }

    async fn verify(
        &self,
        tx: &bastion_core::transaction::NormalizedTransaction,
    ) -> Result<bastion_core::adapter::SimulationOutcome, bastion_core::TrustAdapterError> {
        let evm_tx = EvmTxParams {
            from: tx.from.as_str().to_string(),
            to: tx.to.as_str().to_string(),
            value: Some(format!("0x{:x}", tx.amount)),
            data: tx
                .metadata
                .get("data")
                .and_then(|v| v.as_str())
                .map(ToString::to_string),
            gas: None,
            gas_price: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
            nonce: None,
        };
        // simulate_evm_tx uses a blocking reqwest client; calling it directly from
        // this async fn panics ("cannot drop a runtime in a context where blocking
        // is not allowed"). block_in_place keeps the borrow of `self` but tells the
        // (multi-threaded) tokio scheduler it's safe to block this worker thread.
        let result = tokio::task::block_in_place(|| EvmSimulate::simulate_evm_tx(self, &evm_tx))
            .map_err(|e| bastion_core::TrustAdapterError::SimulationFailed(e.to_string()))?;
        Ok(bastion_core::adapter::SimulationOutcome {
            balance_changes: result
                .balance_changes
                .iter()
                .map(|(k, v)| (k.clone(), *v))
                .collect(),
            logs: result.logs,
            success: result.error.is_none(),
        })
    }

    async fn execute(
        &self,
        _tx: &bastion_core::transaction::NormalizedTransaction,
    ) -> Result<bastion_core::adapter::ExecutionReceipt, bastion_core::TrustAdapterError> {
        // The EVM adapter simulates but does not broadcast; real execution paths are
        // handled by the agent's own wallet/SDK. Return a simulated receipt.
        Ok(bastion_core::adapter::ExecutionReceipt {
            tx_hash: format!("evm:simulated:{}", self.chain_label),
            block_number: None,
            success: true,
        })
    }

    async fn settle(
        &self,
        _receipt: &bastion_core::adapter::ExecutionReceipt,
    ) -> Result<(), bastion_core::TrustAdapterError> {
        Ok(())
    }

    fn chain_name(&self) -> &str {
        &self.chain_label
    }

    fn chain(&self) -> bastion_core::transaction::Chain {
        chain_from_label(&self.chain_label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evm_simulator_creation() {
        let sim = EvmSimulator::from_rpc_url("http://localhost:8545");
        assert_eq!(sim.rpc_url, "http://localhost:8545");
        assert_eq!(sim.chain_label(), "evm");
    }

    #[test]
    fn test_evm_simulator_for_chain_labels() {
        let sim = EvmSimulator::for_chain("ethereum", "http://localhost:8545");
        assert_eq!(sim.chain_label(), "ethereum");
        assert_eq!(sim.rpc_url, "http://localhost:8545");
    }

    #[test]
    fn test_evm_rpc_env_var_mapping() {
        assert_eq!(evm_rpc_env_var("ethereum"), "ETH_RPC_URL");
        assert_eq!(evm_rpc_env_var("base"), "BASE_RPC_URL");
        assert_eq!(evm_rpc_env_var("celo"), "CELO_RPC_URL");
        // Testnet var name is not derivable from the chain name.
        assert_eq!(evm_rpc_env_var("sepolia"), "ETH_SEPOLIA_RPC_URL");
        // zkSync chains
        assert_eq!(evm_rpc_env_var("zksync"), "ZKSYNC_RPC_URL");
        assert_eq!(evm_rpc_env_var("zksync_sepolia"), "ZKSYNC_SEPOLIA_RPC_URL");
        // Robinhood chains
        assert_eq!(evm_rpc_env_var("robinhood"), "ROBINHOOD_RPC_URL");
        assert_eq!(
            evm_rpc_env_var("robinhood_testnet"),
            "ROBINHOOD_TESTNET_RPC_URL"
        );
        // Unknown chains fall back to the derived form.
        assert_eq!(evm_rpc_env_var("polygon"), "POLYGON_RPC_URL");
    }

    #[test]
    fn test_evm_tx_params_serialization() {
        let tx = EvmTxParams {
            from: "0x1234".to_string(),
            to: "0x5678".to_string(),
            value: Some("0xDE0B6B3A7640000".to_string()),
            data: Some("0x".to_string()),
            gas: None,
            gas_price: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
            nonce: None,
        };
        let json = serde_json::to_string(&tx).unwrap();
        assert!(json.contains("0x1234"));
        assert!(json.contains("0xDE0B6B3A7640000"));
    }
}
