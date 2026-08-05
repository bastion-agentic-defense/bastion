//! Manages compensating actions to maintain atomicity when cross-chain plans fail partway

use crate::execution::plan::{ExecutionPlan, LegStatus, PlanStatus};
use uuid::Uuid;

/// Tracks and orchestrates compensating actions when an execution plan
/// encounters a failure after some legs have already completed
#[derive(Debug)]
pub struct AtomicityManager {
    /// Legs whose compensating action finished successfully
    pub compensated: Vec<Uuid>,
    /// Legs whose compensating action is currently in flight
    pub in_progress: Vec<Uuid>,
    /// Legs whose compensating action itself failed (needs manual intervention)
    pub failed: Vec<Uuid>,
}

impl Default for AtomicityManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AtomicityManager {
    pub fn new() -> Self {
        Self {
            compensated: Vec::new(),
            in_progress: Vec::new(),
            failed: Vec::new(),
        }
    }

    /// Checks whether the plan needs compensation
    ///
    /// Returns true only when the plan has at least one Failed leg
    /// and at least one Completed leg that carries a compensating action
    pub fn should_compensate(&self, plan: &ExecutionPlan) -> bool {
        plan.has_failed() && !plan.legs_needing_compensation().is_empty()
    }

    /// Begins the compensation process by transitioning completed legs into
    /// Compensating state in reverse execution order
    ///
    /// Returns the list of leg IDs that need compensation applied
    pub fn begin_compensation(&mut self, plan: &mut ExecutionPlan) -> Vec<Uuid> {
        plan.status = PlanStatus::Compensating;

        let to_compensate: Vec<Uuid> = plan
            .legs
            .iter()
            .rev()
            .filter(|leg| leg.status == LegStatus::Completed && leg.compensating_action.is_some())
            .map(|leg| leg.id)
            .collect();

        for leg_id in &to_compensate {
            if let Some(leg) = plan.legs.iter_mut().find(|l| l.id == *leg_id) {
                leg.status = LegStatus::Compensating;
                self.in_progress.push(*leg_id);
            }
        }

        to_compensate
    }

    /// Marks a leg as successfully compensated and updates the plan status
    /// when all compensation work is done
    pub fn mark_compensated(&mut self, plan: &mut ExecutionPlan, leg_id: Uuid) {
        self.in_progress.retain(|id| *id != leg_id);
        self.compensated.push(leg_id);

        if let Some(leg) = plan.legs.iter_mut().find(|l| l.id == leg_id) {
            leg.status = LegStatus::Compensated;
        }

        if self.in_progress.is_empty() {
            plan.status = PlanStatus::Failed;
        }
    }

    /// Marks a leg's compensating action as failed
    ///
    /// The leg is reverted to Completed so operators can see it still
    /// needs manual reversal
    pub fn mark_compensation_failed(&mut self, plan: &mut ExecutionPlan, leg_id: Uuid) {
        self.in_progress.retain(|id| *id != leg_id);
        self.failed.push(leg_id);

        if let Some(leg) = plan.legs.iter_mut().find(|l| l.id == leg_id) {
            leg.status = LegStatus::Completed;
        }
    }

    /// Returns a snapshot of the current compensation progress
    pub fn compensation_status(&self) -> CompensationStatus {
        CompensationStatus {
            total_compensated: self.compensated.len(),
            total_in_progress: self.in_progress.len(),
            total_failed: self.failed.len(),
        }
    }
}

/// Summary of compensation progress for reporting and observability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompensationStatus {
    pub total_compensated: usize,
    pub total_in_progress: usize,
    pub total_failed: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution::plan::{Action, ActionType, CompensatingAction, Intent, Leg, LegStatus};
    use crate::transaction::{AgentId, Chain};
    use serde_json::json;

    fn make_leg(id: Uuid, status: LegStatus, has_comp: bool) -> Leg {
        Leg {
            id,
            action: Action {
                action_type: ActionType::Swap,
                chain: Chain::Ethereum,
                protocol: "uni".to_string(),
                token: None,
                amount: None,
                destination: None,
                metadata: json!({}),
            },
            depends_on: vec![],
            status,
            compensating_action: if has_comp {
                Some(CompensatingAction {
                    description: "undo".to_string(),
                    chain: Chain::Ethereum,
                    action_type: ActionType::Swap,
                    parameters: json!({}),
                })
            } else {
                None
            },
            result: None,
            error: None,
        }
    }

    fn make_plan(legs: Vec<Leg>) -> ExecutionPlan {
        ExecutionPlan {
            id: Uuid::new_v4(),
            intent: Intent {
                description: "test".to_string(),
                agent_id: AgentId::new("a1"),
                source_chain: Chain::Ethereum,
                actions: vec![],
                constraints: vec![],
            },
            legs,
            status: PlanStatus::Executing,
            created_at: 0,
        }
    }

    #[test]
    fn test_should_compensate_when_failed_with_completed_legs() {
        let plan = make_plan(vec![
            make_leg(Uuid::new_v4(), LegStatus::Completed, true),
            make_leg(Uuid::new_v4(), LegStatus::Failed, false),
        ]);
        let mgr = AtomicityManager::new();
        assert!(mgr.should_compensate(&plan));
    }

    #[test]
    fn test_begin_compensation_marks_legs_in_reverse_order() {
        let id_first = Uuid::new_v4();
        let id_second = Uuid::new_v4();
        let mut plan = make_plan(vec![
            make_leg(id_first, LegStatus::Completed, true),
            make_leg(id_second, LegStatus::Completed, true),
            make_leg(Uuid::new_v4(), LegStatus::Failed, false),
        ]);
        let mut mgr = AtomicityManager::new();
        let to_comp = mgr.begin_compensation(&mut plan);

        assert_eq!(to_comp.len(), 2);
        assert_eq!(to_comp[0], id_second);
        assert_eq!(to_comp[1], id_first);
        assert_eq!(plan.status, PlanStatus::Compensating);

        for leg in &plan.legs[..2] {
            assert_eq!(leg.status, LegStatus::Compensating);
        }
    }

    #[test]
    fn test_compensation_status_tracks_counts() {
        let id_a = Uuid::new_v4();
        let id_b = Uuid::new_v4();
        let mut plan = make_plan(vec![
            make_leg(id_a, LegStatus::Completed, true),
            make_leg(id_b, LegStatus::Completed, true),
            make_leg(Uuid::new_v4(), LegStatus::Failed, false),
        ]);
        let mut mgr = AtomicityManager::new();
        mgr.begin_compensation(&mut plan);

        let status = mgr.compensation_status();
        assert_eq!(status.total_compensated, 0);
        assert_eq!(status.total_in_progress, 2);
        assert_eq!(status.total_failed, 0);

        mgr.mark_compensated(&mut plan, id_a);
        let status = mgr.compensation_status();
        assert_eq!(status.total_compensated, 1);
        assert_eq!(status.total_in_progress, 1);

        mgr.mark_compensation_failed(&mut plan, id_b);
        let status = mgr.compensation_status();
        assert_eq!(status.total_compensated, 1);
        assert_eq!(status.total_in_progress, 0);
        assert_eq!(status.total_failed, 1);
    }
}
