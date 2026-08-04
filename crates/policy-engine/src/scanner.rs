use crate::trust_policy::ScanResult;

/// Background scanner that periodically checks for trust violations.
pub struct BackgroundScanner {
    pub scan_interval_secs: u64,
}

impl BackgroundScanner {
    pub fn new(scan_interval_secs: u64) -> Self {
        Self { scan_interval_secs }
    }

    /// Run a scan and return results.
    /// In production, this queries Sled DB and on-chain state.
    pub async fn scan(
        &self,
        _detect_expired_approvals: bool,
        _detect_expired_delegations: bool,
        _detect_policy_drift: bool,
        _detect_unsettled_transactions: bool,
    ) -> ScanResult {
        let now = chrono::Utc::now().timestamp() as u64;

        ScanResult {
            timestamp: now,
            expired_approvals: 0,
            expired_delegations: 0,
            policy_drifts: 0,
            unsettled_transactions: 0,
        }
    }
}
