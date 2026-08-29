use crate::trust_policy::{BackgroundConfig, ScanFinding, ScanFindingKind, ScanResult};

/// Raw observations the scanner reasons about. The host application (the
/// sidecar) gathers these from its stores before each scan, keeping this crate
/// free of storage concerns.
#[derive(Debug, Clone, Default)]
pub struct ScanSnapshot {
    /// Unix timestamp (seconds) when the snapshot was gathered.
    pub now: u64,
    /// (approval id, created_at) for open human-in-the-loop holds.
    pub pending_approvals: Vec<(String, u64)>,
    /// (agent DID, delegation expiry) for delegations that carry an expiry.
    pub delegation_expiries: Vec<(String, u64)>,
    /// Names of enforced TrustPolicies whose requirements the live engine no
    /// longer satisfies. Computed by the host, which owns both configurations.
    pub drifted_policies: Vec<String>,
    /// (plan id, last activity) for execution plans that have not reached a
    /// settled terminal state.
    pub open_plans: Vec<(String, u64)>,
}

/// Background scanner that checks a snapshot for trust violations.
pub struct BackgroundScanner {
    pub scan_interval_secs: u64,
    /// HITL approvals held longer than this are reported as expired.
    pub approval_ttl_secs: u64,
    /// Plans idle longer than this are reported as unsettled.
    pub unsettled_ttl_secs: u64,
}

impl BackgroundScanner {
    pub fn new(scan_interval_secs: u64) -> Self {
        Self {
            scan_interval_secs,
            approval_ttl_secs: 3600,
            unsettled_ttl_secs: 86_400,
        }
    }

    pub fn with_ttls(mut self, approval_ttl_secs: u64, unsettled_ttl_secs: u64) -> Self {
        self.approval_ttl_secs = approval_ttl_secs;
        self.unsettled_ttl_secs = unsettled_ttl_secs;
        self
    }

    /// Scan a snapshot for violations. `config` selects which detectors run.
    pub fn scan(&self, snapshot: &ScanSnapshot, config: &BackgroundConfig) -> ScanResult {
        let mut findings = Vec::new();

        if config.detect_expired_approvals {
            for (id, created_at) in &snapshot.pending_approvals {
                if snapshot.now.saturating_sub(*created_at) >= self.approval_ttl_secs {
                    findings.push(ScanFinding {
                        kind: ScanFindingKind::ExpiredApproval,
                        id: id.clone(),
                        detail: format!("approval held for more than {}s", self.approval_ttl_secs),
                    });
                }
            }
        }

        if config.detect_expired_delegations {
            for (did, expires_at) in &snapshot.delegation_expiries {
                if *expires_at <= snapshot.now {
                    findings.push(ScanFinding {
                        kind: ScanFindingKind::ExpiredDelegation,
                        id: did.clone(),
                        detail: "delegation expiry has passed".to_string(),
                    });
                }
            }
        }

        if config.detect_policy_drift {
            for name in &snapshot.drifted_policies {
                findings.push(ScanFinding {
                    kind: ScanFindingKind::PolicyDrift,
                    id: name.clone(),
                    detail: "enforced policy requirements are no longer met by the live engine"
                        .to_string(),
                });
            }
        }

        if config.detect_unsettled_transactions {
            for (id, last_activity) in &snapshot.open_plans {
                if snapshot.now.saturating_sub(*last_activity) >= self.unsettled_ttl_secs {
                    findings.push(ScanFinding {
                        kind: ScanFindingKind::UnsettledTransaction,
                        id: id.clone(),
                        detail: format!(
                            "plan idle for more than {}s without settling",
                            self.unsettled_ttl_secs
                        ),
                    });
                }
            }
        }

        let count =
            |kind: ScanFindingKind| findings.iter().filter(|f| f.kind == kind).count() as u64;
        ScanResult {
            timestamp: snapshot.now,
            expired_approvals: count(ScanFindingKind::ExpiredApproval),
            expired_delegations: count(ScanFindingKind::ExpiredDelegation),
            policy_drifts: count(ScanFindingKind::PolicyDrift),
            unsettled_transactions: count(ScanFindingKind::UnsettledTransaction),
            findings,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scanner() -> BackgroundScanner {
        BackgroundScanner::new(3600).with_ttls(100, 1000)
    }

    fn all_on() -> BackgroundConfig {
        BackgroundConfig::default()
    }

    #[test]
    fn empty_snapshot_reports_no_violations() {
        let result = scanner().scan(&ScanSnapshot::default(), &all_on());
        assert_eq!(result.expired_approvals, 0);
        assert_eq!(result.expired_delegations, 0);
        assert_eq!(result.policy_drifts, 0);
        assert_eq!(result.unsettled_transactions, 0);
        assert!(result.findings.is_empty());
    }

    #[test]
    fn expired_approvals_respect_ttl_boundary() {
        let snapshot = ScanSnapshot {
            now: 10_000,
            pending_approvals: vec![
                ("fresh".into(), 9_950), // 50s old, TTL 100
                ("stale".into(), 9_800), // 200s old
            ],
            ..Default::default()
        };
        let result = scanner().scan(&snapshot, &all_on());
        assert_eq!(result.expired_approvals, 1);
        assert_eq!(result.findings[0].id, "stale");
        assert_eq!(result.findings[0].kind, ScanFindingKind::ExpiredApproval);
    }

    #[test]
    fn disabled_detectors_report_nothing() {
        let snapshot = ScanSnapshot {
            now: 10_000,
            pending_approvals: vec![("stale".into(), 0)],
            delegation_expiries: vec![("did:bastion:a".into(), 5)],
            drifted_policies: vec!["policy-a".into()],
            open_plans: vec![("plan-1".into(), 0)],
        };
        let config = BackgroundConfig {
            detect_expired_approvals: false,
            detect_expired_delegations: false,
            detect_policy_drift: false,
            detect_unsettled_transactions: false,
            ..Default::default()
        };
        let result = scanner().scan(&snapshot, &config);
        assert_eq!(result.expired_approvals, 0);
        assert_eq!(result.expired_delegations, 0);
        assert_eq!(result.policy_drifts, 0);
        assert_eq!(result.unsettled_transactions, 0);
        assert!(result.findings.is_empty());
    }

    #[test]
    fn counts_each_detector_independently() {
        let snapshot = ScanSnapshot {
            now: 10_000,
            pending_approvals: vec![("stale".into(), 0)],
            delegation_expiries: vec![
                ("did:bastion:expired".into(), 9_000),
                ("did:bastion:active".into(), 20_000),
            ],
            drifted_policies: vec!["policy-a".into(), "policy-b".into()],
            open_plans: vec![("plan-old".into(), 5_000), ("plan-new".into(), 9_950)],
        };
        let result = scanner().scan(&snapshot, &all_on());
        assert_eq!(result.expired_approvals, 1);
        assert_eq!(result.expired_delegations, 1);
        assert_eq!(result.policy_drifts, 2);
        assert_eq!(result.unsettled_transactions, 1);
        assert_eq!(result.findings.len(), 5);
    }
}
