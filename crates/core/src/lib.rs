//! Bastion Core - chain-agnostic types and policy engine.
//!
//! This crate provides the shared foundation for all chain-specific
//! Bastion implementations:
//!
//! - `NormalizedTransaction`: chain-agnostic transaction representation
//! - `FirewallDecision`: the outcome of a policy evaluation
//! - `PolicyEvaluator`: core evaluation loop
//! - `TrustSignalProvider`: trait for consuming trust signals from ARES or other providers
//! - `TrustAdapter`: trait for chain-independent execution environments
//! - `AuditRecord`: chain-agnostic audit event

pub mod adapter;
pub mod audit;
pub mod decision;
pub mod execution;
pub mod policy;
pub mod risk;
pub mod transaction;

pub use adapter::{
    AgentIdentity, ExecutionReceipt, SimulationOutcome, TrustAdapter, TrustAdapterError,
};
pub use audit::AuditRecord;
pub use decision::FirewallDecision;
pub use policy::{PolicyEvaluator, PolicyRule, PolicySet};
pub use risk::{RiskScore, TrustSignalError, TrustSignalProvider, WebacyClient};
pub use transaction::{Address, AgentId, Chain, NormalizedTransaction, TxType};
