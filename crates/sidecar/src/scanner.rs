//! Background trust scanner.
//!
//! Periodically sweeps sidecar state for trust violations and records the
//! result for `GET /scan/results`. Detectors: expired HITL approvals, expired
//! agent delegations, policy drift (an enforced TrustPolicy whose requirements
//! the live engine no longer meets), and unsettled execution plans.
//!
//! Configuration via environment variables:
//! - `BASTION_SCAN_INTERVAL_SECS` (default 3600) - period of the background loop
//! - `BASTION_SCAN_APPROVAL_TTL_SECS` (default 3600) - approval age before expiry
//! - `BASTION_SCAN_UNSETTLED_TTL_SECS` (default 86400) - plan idle age before unsettled

use bastion_policy_engine::trust_policy::{BackgroundConfig, PolicyMode, ScanResult};
use bastion_policy_engine::{BackgroundScanner, ScanSnapshot};
use bastion_workflow::state::WorkflowStatus;

use crate::AppState;
use crate::audit::current_timestamp;
use crate::policy::Policy;

fn env_u64(key: &str, default: u64) -> u64 {
    std::env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn scan_interval_secs() -> u64 {
    env_u64("BASTION_SCAN_INTERVAL_SECS", 3600)
}

fn approval_ttl_secs() -> u64 {
    env_u64("BASTION_SCAN_APPROVAL_TTL_SECS", 3600)
}

fn unsettled_ttl_secs() -> u64 {
    env_u64("BASTION_SCAN_UNSETTLED_TTL_SECS", 86_400)
}

/// Spawn the periodic scanning loop. The first scan runs after one full
/// interval; `POST /scans` triggers an immediate scan on demand.
pub(crate) fn spawn(state: AppState) {
    let interval = scan_interval_secs();
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(std::time::Duration::from_secs(interval));
        // interval() fires immediately on the first tick; skip it so the first
        // scan runs one interval after boot.
        ticker.tick().await;
        loop {
            ticker.tick().await;
            run_scan(&state).await;
        }
    });
}

/// A TrustPolicy in Enforce mode drifts when the live engine configuration no
/// longer satisfies the requirements the policy declares. Only checks the
/// engine can actually enforce are compared: destination blocklists and the
/// per-transaction value cap.
fn policy_has_drifted(policy: &bastion_policy_engine::TrustPolicy, live: &Policy) -> bool {
    let v = &policy.spec.validate;

    // Every blocklisted address must be blocked by the live engine.
    if v.blocklist
        .iter()
        .any(|a| !live.blocked_addresses.contains(a))
    {
        return true;
    }

    // A per-transaction cap requires a live cap at least as strict.
    if let Some(cap) = v.max_per_transaction {
        match live.max_sol_per_tx {
            Some(live_cap) if live_cap <= cap => {}
            _ => return true,
        }
    }

    false
}

/// Run one scan over the current sidecar state, store the result, and
/// broadcast it on the SSE event stream.
pub(crate) async fn run_scan(state: &AppState) -> ScanResult {
    let now = current_timestamp();

    // 1. Open HITL approvals with their creation time.
    let pending_approvals: Vec<(String, u64)> = state
        .pending_approvals
        .read()
        .await
        .iter()
        .map(|(id, p)| (id.clone(), p.created_at))
        .collect();

    // 2. Delegations that carry an expiry.
    let delegation_expiries: Vec<(String, u64)> = state
        .agent_store
        .list_agents()
        .unwrap_or_default()
        .into_iter()
        .filter_map(|a| {
            a.delegation_expires_at
                .and_then(|e| u64::try_from(e).ok())
                .map(|e| (a.did, e))
        })
        .collect();

    // 3. Drift between enforced TrustPolicies and the live engine.
    let drifted_policies: Vec<String> = {
        let engine = state.policy_engine.read().await;
        let live = engine.policy_snapshot();
        let stores = state.policy_stores.read().await;
        stores
            .iter()
            .filter(|p| matches!(p.mode(), PolicyMode::Enforce))
            .filter(|p| policy_has_drifted(&p.policy, &live))
            .map(|p| p.policy.metadata.name.clone())
            .collect()
    };

    // 4. Execution plans that have not settled: still running or paused, or
    //    failed with no compensation started.
    let open_plans: Vec<(String, u64)> = {
        let plans = state.execution_plans.read().await;
        plans
            .iter()
            .filter_map(|(id, tracked)| {
                let wf = state.workflow_engine.state(&tracked.workflow_id).ok()?;
                let unsettled = match &wf.status {
                    WorkflowStatus::Running | WorkflowStatus::Paused => true,
                    WorkflowStatus::Failed(_) => tracked.compensation_workflow_id.is_none(),
                    _ => false,
                };
                unsettled.then(|| (id.clone(), wf.updated_at))
            })
            .collect()
    };

    let snapshot = ScanSnapshot {
        now,
        pending_approvals,
        delegation_expiries,
        drifted_policies,
        open_plans,
    };
    let scanner = BackgroundScanner::new(scan_interval_secs())
        .with_ttls(approval_ttl_secs(), unsettled_ttl_secs());
    let result = scanner.scan(&snapshot, &BackgroundConfig::default());

    *state.last_scan.write().await = Some(result.clone());
    crate::emit_event(
        &state.event_tx,
        "trust_scan",
        &serde_json::to_string(&result).unwrap_or_default(),
    );
    result
}
