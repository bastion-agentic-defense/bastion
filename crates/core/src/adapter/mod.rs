//! TrustAdapter — chain-independence abstraction for the Bastion runtime.
//!
//! Each execution environment (Ethereum, zkSync, Solana, Arcium, Midnight)
//! implements this trait, making Bastion's policy engine chain-independent.
//! The runtime composes adapters via the trait, never importing chain-specific
//! SDKs directly.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::transaction::{Address, AgentId, Chain, NormalizedTransaction};

/// Result of authenticating an agent on a specific chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentIdentity {
    pub agent_id: AgentId,
    pub chain: Chain,
    pub address: Address,
    pub reputation: Option<u8>,
}

/// Predicted outcome of simulating a transaction before execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationOutcome {
    pub balance_changes: HashMap<String, i64>,
    pub logs: Vec<String>,
    pub success: bool,
}

/// Receipt after a transaction is executed on-chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionReceipt {
    pub tx_hash: String,
    pub block_number: Option<u64>,
    pub success: bool,
}

/// TrustAdapter abstracts chain-specific operations so that Bastion's
/// runtime is chain-independent. Each execution environment (Ethereum,
/// zkSync, Solana, Arcium, Midnight) implements this trait.
///
/// The five-phase trust pipeline:
///
/// ```text
/// authenticate → authorize → verify → execute → settle
/// ```
///
/// - **authenticate**: resolve an on-chain address to a Bastion agent identity
/// - **authorize**: evaluate a normalized transaction against policy (uses PolicyEvaluator)
/// - **verify**: simulate the transaction and predict its outcome
/// - **execute**: submit the transaction on-chain and return a receipt
/// - **settle**: record the result in the audit log (on-chain or off-chain)
#[async_trait]
pub trait TrustAdapter: Send + Sync {
    /// Resolve the on-chain identity of an agent address.
    async fn authenticate(&self, address: &Address) -> Result<AgentIdentity, TrustAdapterError>;

    /// Simulate a normalized transaction and predict balance changes.
    async fn verify(
        &self,
        tx: &NormalizedTransaction,
    ) -> Result<SimulationOutcome, TrustAdapterError>;

    /// Execute a transaction on-chain and return the receipt.
    async fn execute(
        &self,
        tx: &NormalizedTransaction,
    ) -> Result<ExecutionReceipt, TrustAdapterError>;

    /// Record the execution result in the audit log.
    async fn settle(&self, receipt: &ExecutionReceipt) -> Result<(), TrustAdapterError>;

    /// Human-readable name of the chain this adapter targets (e.g. "ethereum").
    fn chain_name(&self) -> &str;

    /// The Chain variant this adapter corresponds to.
    fn chain(&self) -> Chain;
}

/// Errors that can occur during trust adapter operations.
#[derive(Debug, thiserror::Error)]
pub enum TrustAdapterError {
    #[error("authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("simulation failed: {0}")]
    SimulationFailed(String),

    #[error("execution failed: {0}")]
    ExecutionFailed(String),

    #[error("settlement failed: {0}")]
    SettlementFailed(String),

    #[error("chain not supported: {0}")]
    UnsupportedChain(String),

    #[error("rpc error: {0}")]
    RpcError(String),
}
