//! Execution plan types for cross-chain settlement routing

use crate::transaction::{Address, AgentId, Chain};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The kind of on-chain action to execute
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionType {
    Swap,
    Transfer,
    Bridge,
    Stake,
    Unstake,
    Lend,
    Borrow,
    DeployContract,
    CallContract,
    Custom(String),
}

/// A single on-chain action with all parameters needed for execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub action_type: ActionType,
    pub chain: Chain,
    pub protocol: String,
    pub token: Option<String>,
    pub amount: Option<u64>,
    pub destination: Option<Address>,
    pub metadata: serde_json::Value,
}

/// The kind of constraint that applies to plan execution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstraintType {
    MaxSlippageBps(u16),
    MaxGasCost(u64),
    Deadline(u64),
    MinAmount(u64),
    MaxAmount(u64),
    RequiredConfirmations(u8),
    WhitelistedProtocols(Vec<String>),
    BlacklistedProtocols(Vec<String>),
}

/// A constraint that limits or bounds how an action may execute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub constraint_type: ConstraintType,
    /// When true the constraint is a hard requirement and must not be violated
    pub hard: bool,
    pub description: String,
}

/// High-level intent from an agent describing a desired outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub description: String,
    pub agent_id: AgentId,
    pub source_chain: Chain,
    pub actions: Vec<Action>,
    pub constraints: Vec<Constraint>,
}

/// Status of an individual execution leg
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegStatus {
    /// Waiting for upstream legs to complete
    Pending,
    /// Running pre-flight validation
    Validating,
    /// Running dry-run simulation
    Simulating,
    /// All prerequisites satisfied, ready to submit
    Ready,
    /// Transaction submitted and awaiting confirmation
    Executing,
    /// Transaction confirmed successfully
    Completed,
    /// Transaction failed irrecoverably
    Failed,
    /// Running the compensating action to undo this leg
    Compensating,
    /// Compensating action completed
    Compensated,
    /// Leg was skipped (for example an optional branch not taken)
    Skipped,
}

/// Status of the entire execution plan
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlanStatus {
    Draft,
    Validated,
    Executing,
    Completed,
    Failed,
    PartiallyCompleted,
    Compensating,
    Cancelled,
}

/// An action that reverses or undoes a completed leg on failure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensatingAction {
    pub description: String,
    pub chain: Chain,
    pub action_type: ActionType,
    pub parameters: serde_json::Value,
}

/// A single leg in an execution plan representing one discrete step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Leg {
    pub id: Uuid,
    pub action: Action,
    /// IDs of legs that must complete before this one can start
    pub depends_on: Vec<Uuid>,
    pub status: LegStatus,
    /// Optional compensating action to undo this leg on failure
    pub compensating_action: Option<CompensatingAction>,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
}

/// A complete execution plan with ordered legs and lifecycle tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub id: Uuid,
    pub intent: Intent,
    pub legs: Vec<Leg>,
    pub status: PlanStatus,
    /// Unix timestamp in seconds when the plan was created
    pub created_at: u64,
}

impl ExecutionPlan {
    /// Creates a new draft plan from the given intent with no legs
    pub fn new(intent: Intent) -> Self {
        Self {
            id: Uuid::new_v4(),
            intent,
            legs: Vec::new(),
            status: PlanStatus::Draft,
            created_at: 0,
        }
    }

    /// Returns legs that are either Ready or whose dependencies are all Completed
    pub fn ready_legs(&self) -> Vec<&Leg> {
        self.legs
            .iter()
            .filter(|leg| {
                leg.status == LegStatus::Ready
                    || (leg.status == LegStatus::Pending
                        && leg.depends_on.iter().all(|dep_id| {
                            self.legs
                                .iter()
                                .any(|l| l.id == *dep_id && l.status == LegStatus::Completed)
                        }))
            })
            .collect()
    }

    /// True when every leg is either Completed or Skipped
    pub fn is_complete(&self) -> bool {
        !self.legs.is_empty()
            && self
                .legs
                .iter()
                .all(|l| l.status == LegStatus::Completed || l.status == LegStatus::Skipped)
    }

    /// True when any leg has entered the Failed state
    pub fn has_failed(&self) -> bool {
        self.legs.iter().any(|l| l.status == LegStatus::Failed)
    }

    /// Returns legs that completed successfully but carry a compensating action
    /// and therefore need to be reversed after a downstream failure
    pub fn legs_needing_compensation(&self) -> Vec<&Leg> {
        if !self.has_failed() {
            return Vec::new();
        }
        self.legs
            .iter()
            .filter(|l| l.status == LegStatus::Completed && l.compensating_action.is_some())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn make_agent_id() -> AgentId {
        AgentId::new("agent-001")
    }

    fn make_action(chain: Chain, action_type: ActionType) -> Action {
        Action {
            action_type,
            chain,
            protocol: "uniswap".to_string(),
            token: Some("USDC".to_string()),
            amount: Some(1_000_000),
            destination: Some(Address::new("0xabc")),
            metadata: json!({}),
        }
    }

    fn make_leg(id: Uuid, depends_on: Vec<Uuid>, status: LegStatus) -> Leg {
        Leg {
            id,
            action: make_action(Chain::Ethereum, ActionType::Swap),
            depends_on,
            status,
            compensating_action: Some(CompensatingAction {
                description: "reverse swap".to_string(),
                chain: Chain::Ethereum,
                action_type: ActionType::Swap,
                parameters: json!({}),
            }),
            result: None,
            error: None,
        }
    }

    fn make_intent() -> Intent {
        Intent {
            description: "swap USDC for ETH on Ethereum".to_string(),
            agent_id: make_agent_id(),
            source_chain: Chain::Ethereum,
            actions: vec![make_action(Chain::Ethereum, ActionType::Swap)],
            constraints: Vec::new(),
        }
    }

    #[test]
    fn test_new_plan_starts_as_draft() {
        let plan = ExecutionPlan::new(make_intent());
        assert_eq!(plan.status, PlanStatus::Draft);
        assert!(plan.legs.is_empty());
    }

    #[test]
    fn test_ready_legs_returns_legs_with_met_dependencies() {
        let id_a = Uuid::new_v4();
        let id_b = Uuid::new_v4();
        let plan = ExecutionPlan {
            id: Uuid::new_v4(),
            intent: make_intent(),
            legs: vec![
                make_leg(id_a, vec![], LegStatus::Completed),
                make_leg(id_b, vec![id_a], LegStatus::Pending),
            ],
            status: PlanStatus::Executing,
            created_at: 0,
        };
        let ready = plan.ready_legs();
        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0].id, id_b);
    }

    #[test]
    fn test_is_complete_all_done() {
        let plan = ExecutionPlan {
            id: Uuid::new_v4(),
            intent: make_intent(),
            legs: vec![
                make_leg(Uuid::new_v4(), vec![], LegStatus::Completed),
                make_leg(Uuid::new_v4(), vec![], LegStatus::Skipped),
            ],
            status: PlanStatus::Completed,
            created_at: 0,
        };
        assert!(plan.is_complete());
    }

    #[test]
    fn test_is_complete_with_pending_leg() {
        let plan = ExecutionPlan {
            id: Uuid::new_v4(),
            intent: make_intent(),
            legs: vec![
                make_leg(Uuid::new_v4(), vec![], LegStatus::Completed),
                make_leg(Uuid::new_v4(), vec![], LegStatus::Pending),
            ],
            status: PlanStatus::Executing,
            created_at: 0,
        };
        assert!(!plan.is_complete());
    }

    #[test]
    fn test_has_failed_detects_failure() {
        let plan = ExecutionPlan {
            id: Uuid::new_v4(),
            intent: make_intent(),
            legs: vec![
                make_leg(Uuid::new_v4(), vec![], LegStatus::Completed),
                make_leg(Uuid::new_v4(), vec![], LegStatus::Failed),
            ],
            status: PlanStatus::Failed,
            created_at: 0,
        };
        assert!(plan.has_failed());
    }

    #[test]
    fn test_legs_needing_compensation_filters_correctly() {
        let id_done = Uuid::new_v4();
        let id_failed = Uuid::new_v4();
        let id_no_comp = Uuid::new_v4();
        let mut leg_no_comp = make_leg(id_no_comp, vec![], LegStatus::Completed);
        leg_no_comp.compensating_action = None;

        let plan = ExecutionPlan {
            id: Uuid::new_v4(),
            intent: make_intent(),
            legs: vec![
                make_leg(id_done, vec![], LegStatus::Completed),
                make_leg(id_failed, vec![], LegStatus::Failed),
                leg_no_comp,
            ],
            status: PlanStatus::Failed,
            created_at: 0,
        };
        let need_comp = plan.legs_needing_compensation();
        assert_eq!(need_comp.len(), 1);
        assert_eq!(need_comp[0].id, id_done);
    }
}
