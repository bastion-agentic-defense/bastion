use crate::trust_policy::{PolicyMode, PolicyStatus, TrustPolicy};

/// Policy lifecycle: Author -> Test -> DryRun -> Audit -> Enforce
pub struct PolicyLifecycle {
    pub policy: TrustPolicy,
    pub current_mode: PolicyMode,
}

impl PolicyLifecycle {
    pub fn new(policy: TrustPolicy) -> Self {
        Self {
            current_mode: PolicyMode::Audit,
            policy,
        }
    }

    /// Promote from Audit to Enforce mode.
    pub fn enforce(&mut self) {
        self.current_mode = PolicyMode::Enforce;
    }

    /// Downgrade to Audit mode (log, don't block).
    pub fn audit(&mut self) {
        self.current_mode = PolicyMode::Audit;
    }

    pub fn mode(&self) -> &PolicyMode {
        &self.current_mode
    }

    /// Produce a status snapshot.
    pub fn status(&self) -> PolicyStatus {
        self.policy.status.clone().unwrap_or(PolicyStatus {
            mode: self.current_mode.clone(),
            evaluations: 0,
            blocks: 0,
            last_evaluation: 0,
            last_scan: None,
        })
    }
}
