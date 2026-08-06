//! Decomposes high-level intents into executable plans with dependency ordering and compensating actions

use crate::execution::plan::{
    ActionType, CompensatingAction, ExecutionPlan, Intent, Leg, LegStatus,
};
use uuid::Uuid;

/// Turns an Intent into an ExecutionPlan with sequential legs and auto-generated compensating actions
pub struct IntentDecomposer;

impl Default for IntentDecomposer {
    fn default() -> Self {
        Self
    }
}

impl IntentDecomposer {
    pub fn new() -> Self {
        Self
    }

    /// Decomposes an intent into an execution plan where each action becomes a leg
    /// and legs are chained sequentially
    ///
    /// The first leg starts in Ready state. Subsequent legs start as Pending
    /// and depend on the preceding leg.
    pub fn decompose(&self, intent: Intent) -> ExecutionPlan {
        let mut plan = ExecutionPlan::new(intent.clone());
        let mut previous_leg_id: Option<Uuid> = None;

        for action in &intent.actions {
            let leg_id = Uuid::new_v4();
            let mut depends_on = Vec::new();

            if let Some(prev) = previous_leg_id {
                depends_on.push(prev);
            }

            let compensating = self.build_compensating_action(action);
            let initial_status = if depends_on.is_empty() {
                LegStatus::Ready
            } else {
                LegStatus::Pending
            };

            let leg = Leg {
                id: leg_id,
                action: action.clone(),
                depends_on,
                status: initial_status,
                compensating_action: compensating,
                result: None,
                error: None,
            };

            plan.legs.push(leg);
            previous_leg_id = Some(leg_id);
        }

        plan
    }

    /// Builds the appropriate compensating action for a given action type
    fn build_compensating_action(
        &self,
        action: &crate::execution::plan::Action,
    ) -> Option<CompensatingAction> {
        let (description, reverse_type) = match &action.action_type {
            ActionType::Swap => (
                "Reverse swap to original token".to_string(),
                ActionType::Swap,
            ),
            ActionType::Transfer => ("Return transferred funds".to_string(), ActionType::Transfer),
            ActionType::Bridge => (
                "Bridge assets back to source chain".to_string(),
                ActionType::Bridge,
            ),
            ActionType::Stake => (
                "Unstake to reverse position".to_string(),
                ActionType::Unstake,
            ),
            ActionType::Lend => ("Withdraw lent assets".to_string(), ActionType::Lend),
            ActionType::Borrow => ("Repay borrowed assets".to_string(), ActionType::Borrow),
            ActionType::Unstake => return None,
            ActionType::DeployContract => return None,
            ActionType::CallContract => return None,
            ActionType::Custom(_) => return None,
        };

        Some(CompensatingAction {
            description,
            chain: action.chain,
            action_type: reverse_type,
            parameters: serde_json::json!({}),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution::plan::{Action, Intent};
    use crate::transaction::{Address, AgentId, Chain};
    use serde_json::json;

    fn make_action(chain: Chain, at: ActionType) -> Action {
        Action {
            action_type: at,
            chain,
            protocol: "test".to_string(),
            token: Some("USDC".to_string()),
            amount: Some(100),
            destination: Some(Address::new("0xdead")),
            metadata: json!({}),
        }
    }

    #[test]
    fn test_decompose_creates_sequential_legs() {
        let intent = Intent {
            description: "swap then bridge then stake".to_string(),
            agent_id: AgentId::new("agent-1"),
            source_chain: Chain::Ethereum,
            actions: vec![
                make_action(Chain::Ethereum, ActionType::Swap),
                make_action(Chain::Base, ActionType::Bridge),
                make_action(Chain::Base, ActionType::Stake),
            ],
            constraints: Vec::new(),
        };

        let decomposer = IntentDecomposer::new();
        let plan = decomposer.decompose(intent);

        assert_eq!(plan.legs.len(), 3);
        assert!(plan.legs[0].depends_on.is_empty());
        assert_eq!(plan.legs[0].status, LegStatus::Ready);

        assert_eq!(plan.legs[1].depends_on, vec![plan.legs[0].id]);
        assert_eq!(plan.legs[1].status, LegStatus::Pending);

        assert_eq!(plan.legs[2].depends_on, vec![plan.legs[1].id]);
        assert_eq!(plan.legs[2].status, LegStatus::Pending);
    }

    #[test]
    fn test_decompose_generates_compensating_actions() {
        let intent = Intent {
            description: "swap and transfer".to_string(),
            agent_id: AgentId::new("agent-2"),
            source_chain: Chain::Solana,
            actions: vec![
                make_action(Chain::Solana, ActionType::Swap),
                make_action(Chain::Solana, ActionType::Transfer),
            ],
            constraints: Vec::new(),
        };

        let decomposer = IntentDecomposer::new();
        let plan = decomposer.decompose(intent);

        assert!(plan.legs[0].compensating_action.is_some());
        assert!(plan.legs[1].compensating_action.is_some());
        assert_eq!(
            plan.legs[0]
                .compensating_action
                .as_ref()
                .unwrap()
                .action_type,
            ActionType::Swap
        );
    }

    #[test]
    fn test_decompose_skips_compensation_for_contract_actions() {
        let intent = Intent {
            description: "deploy and call".to_string(),
            agent_id: AgentId::new("agent-3"),
            source_chain: Chain::Base,
            actions: vec![
                make_action(Chain::Base, ActionType::DeployContract),
                make_action(Chain::Base, ActionType::CallContract),
            ],
            constraints: Vec::new(),
        };

        let decomposer = IntentDecomposer::new();
        let plan = decomposer.decompose(intent);

        assert!(plan.legs[0].compensating_action.is_none());
        assert!(plan.legs[1].compensating_action.is_none());
    }
}
