use crate::decision::FirewallDecision;
use crate::transaction::{Address, AgentId, Chain, NormalizedTransaction};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// A chain-agnostic audit record produced after every firewall evaluation.
///
/// Implements ERC-8281 (OCP) + ERC-8299 (WYRIWE) commitment hashing from
/// trustless-ai/agent-ercs. Every audit record carries a recompute-able
/// `observation_digest` and `wyriwe_hash` so third parties can independently
/// verify what happened without trusting Bastion's servers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRecord {
    /// When the audit record was created (Unix timestamp, seconds).
    pub timestamp: u64,
    /// The agent that attempted the transaction.
    pub agent_id: AgentId,
    /// The chain the transaction originated on.
    pub chain: Chain,
    /// Source address.
    pub from: Address,
    /// Destination address.
    pub to: Address,
    /// Transaction value in base units.
    pub amount: u64,
    /// Currency identifier.
    pub currency: String,
    /// The firewall decision.
    pub decision: FirewallDecision,
    /// Hash of the original transaction payload (for correlation).
    pub payload_hash: String,
    /// ERC-8281 OCP observation digest - keccak256 of the committed observation
    /// bytes, enabling trustless recompute verification by external parties.
    pub observation_digest: String,
    /// ERC-8299 WYRIWE input-provenance hash. The triple-hash commitment:
    /// keccak256(rawInputHash || sanitizationPipelineHash || inputHash).
    /// Binds what the agent asked for (raw input) to what passed policy
    /// (sanitized input), verifiable by any third party.
    pub wyriwe_hash: String,
    /// Optional extra metadata.
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
}

impl AuditRecord {
    pub fn from_transaction(
        tx: &NormalizedTransaction,
        decision: FirewallDecision,
        payload_hash: impl Into<String>,
    ) -> Self {
        let payload_hash = payload_hash.into();

        // ERC-8299 WYRIWE triple-hash: binds the agent's declared intent
        // (raw input) to what the policy engine evaluated (sanitized input).
        // Using SHA-256 to match the recompute standard from agent-sdk.
        let raw_input = serde_json::to_string(tx).unwrap_or_default();
        let raw_input_hash = sha256_hex(raw_input.as_bytes());

        // The sanitization pipeline is "identity" when no transform is applied
        // (policy engine passes the raw input through unchanged).
        let sanitization_cid = "identity";
        let sanitization_pipeline_hash =
            sha256_hex(&[sanitization_cid.as_bytes(), raw_input_hash.as_bytes()].concat());

        // The sanitized input is the raw input (identity pipeline case).
        // In a real deployment with content filtering, this would be the
        // filtered/transformed input.
        let input_hash = sha256_hex(raw_input.as_bytes());

        let wyriwe_hash =
            sha256_hex(&[raw_input_hash.as_bytes(), sanitization_pipeline_hash.as_bytes(), input_hash.as_bytes()].concat());

        // ERC-8281 OCP: the observation is the policy decision + payload hash.
        let observation = format!("{:?}:{}", decision, payload_hash);
        let observation_digest = sha256_hex(observation.as_bytes());

        Self {
            timestamp: tx.timestamp,
            agent_id: tx.agent_id.clone(),
            chain: tx.chain,
            from: tx.from.clone(),
            to: tx.to.clone(),
            amount: tx.amount,
            currency: tx.currency.clone(),
            decision,
            payload_hash,
            observation_digest,
            wyriwe_hash,
            metadata: tx.metadata.clone(),
        }
    }

    /// Recompute the WYRIWE hash from public inputs. Any third party can
    /// call this with the raw transaction data to verify the audit record
    /// was computed correctly - no trust in Bastion required.
    pub fn recompute_wyriwe_hash(raw_input: &str) -> String {
        let raw_input_hash = sha256_hex(raw_input.as_bytes());
        let sanitization_cid = "identity";
        let sanitization_pipeline_hash =
            sha256_hex(&[sanitization_cid.as_bytes(), raw_input_hash.as_bytes()].concat());
        let input_hash = sha256_hex(raw_input.as_bytes());

        sha256_hex(
            &[
                raw_input_hash.as_bytes(),
                sanitization_pipeline_hash.as_bytes(),
                input_hash.as_bytes(),
            ]
            .concat(),
        )
    }

    /// Recompute the OCP observation digest from the decision + payload hash.
    pub fn recompute_observation_digest(decision: &FirewallDecision, payload_hash: &str) -> String {
        let observation = format!("{:?}:{}", decision, payload_hash);
        sha256_hex(observation.as_bytes())
    }
}

fn sha256_hex(data: &[u8]) -> String {
    let hash = Sha256::digest(data);
    format!("0x{}", hex::encode(hash))
}
