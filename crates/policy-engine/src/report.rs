use serde::{Deserialize, Serialize};

use crate::trust_policy::{PolicyMode, ScanResult};

/// A TrustReport summarizing policy compliance for an agent or policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrustReport {
    pub policy_name: String,
    pub mode: PolicyMode,
    pub evaluations: u64,
    pub allowed: u64,
    pub blocked: u64,
    pub pending_hitl: u64,
    pub active_exceptions: u64,
    pub last_scan: Option<ScanResult>,
    pub compliant: bool,
}

impl TrustReport {
    pub fn new(policy_name: impl Into<String>, mode: PolicyMode) -> Self {
        Self {
            policy_name: policy_name.into(),
            mode,
            evaluations: 0,
            allowed: 0,
            blocked: 0,
            pending_hitl: 0,
            active_exceptions: 0,
            last_scan: None,
            compliant: true,
        }
    }

    pub fn mark_non_compliant(&mut self) {
        self.compliant = false;
    }
}
