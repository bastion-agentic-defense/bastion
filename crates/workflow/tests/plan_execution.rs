//! End-to-end settlement tests: decompose an intent into a plan, route it,
//! execute it with the durable workflow engine, and reconcile compensation.

use std::time::Duration;

use async_trait::async_trait;
use bastion_core::execution::plan::{Action, ActionType, Intent, LegStatus, PlanStatus};
use bastion_core::execution::{AtomicityManager, ChainMetrics, IntentDecomposer, RouteSelector};
use bastion_core::transaction::{Address, AgentId, Chain};
use bastion_workflow::activities::{
    approve::ApproveActivity, compensate::CompensateActivity, settle::SettleActivity,
    simulate::SimulateActivity, simulate_evm::SimulateEvmActivity,
};
use bastion_workflow::plan_adapter::{
    CompensationWorkflow, PlanAdapterOptions, PlanWorkflow, sync_compensation_from_workflow,
    sync_plan_from_workflow,
};
use bastion_workflow::state::{StepStatus, WorkflowStatus};
use bastion_workflow::{
    Activity, ActivityContext, RetryPolicy, Signal, WorkflowDefinition, WorkflowEngine,
    WorkflowError,
};

fn action(chain: Chain, at: ActionType) -> Action {
    Action {
        action_type: at,
        chain,
        protocol: "test".to_string(),
        token: Some("USDC".to_string()),
        amount: Some(1000),
        destination: Some(Address::new("0xabc")),
        metadata: serde_json::json!({}),
    }
}

fn swap_bridge_stake_intent() -> Intent {
    Intent {
        description: "swap then bridge then stake".to_string(),
        agent_id: AgentId::new("agent-001"),
        source_chain: Chain::Ethereum,
        actions: vec![
            action(Chain::Ethereum, ActionType::Swap),
            action(Chain::Base, ActionType::Bridge),
            action(Chain::Base, ActionType::Stake),
        ],
        constraints: Vec::new(),
    }
}

fn fast_options(require_approval: bool) -> PlanAdapterOptions {
    PlanAdapterOptions {
        require_approval,
        retry: RetryPolicy {
            max_attempts: 1,
            initial_backoff_ms: 5,
            max_backoff_ms: 20,
            backoff_multiplier: 2.0,
            timeout_ms: 1000,
        },
        timeout_ms: 2000,
        approval_timeout_ms: 60_000,
    }
}

fn register_std_activities(engine: &mut WorkflowEngine) {
    engine.register_activity(SimulateActivity);
    engine.register_activity(SimulateEvmActivity);
    engine.register_activity(ApproveActivity);
    engine.register_activity(SettleActivity);
    engine.register_activity(CompensateActivity);
}

async fn wait_until(
    engine: &WorkflowEngine,
    workflow_id: &str,
    predicate: impl Fn(&WorkflowStatus) -> bool,
) -> bastion_workflow::state::WorkflowState {
    for _ in 0..400 {
        let st = engine.state(workflow_id).expect("read state");
        if predicate(&st.status) {
            return st;
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
    panic!("workflow {workflow_id} never satisfied the wait condition");
}

async fn wait_terminal(
    engine: &WorkflowEngine,
    workflow_id: &str,
) -> bastion_workflow::state::WorkflowState {
    wait_until(engine, workflow_id, |s| {
        !matches!(s, WorkflowStatus::Running | WorkflowStatus::Paused)
    })
    .await
}

#[tokio::test]
async fn routes_decomposes_and_executes_cross_chain_intent() {
    // 1. Route: score candidate chains from metrics.
    let mut selector = RouteSelector::new();
    selector.add_chain(
        Chain::Ethereum,
        ChainMetrics {
            cost_per_tx: 4.0,
            latency_ms: 12_000,
            reliability_score: 0.99,
            congestion_level: 0.6,
        },
    );
    selector.add_chain(
        Chain::Base,
        ChainMetrics {
            cost_per_tx: 0.01,
            latency_ms: 2_000,
            reliability_score: 0.97,
            congestion_level: 0.2,
        },
    );
    selector.add_chain(
        Chain::Solana,
        ChainMetrics {
            cost_per_tx: 0.001,
            latency_ms: 400,
            reliability_score: 0.95,
            congestion_level: 0.2,
        },
    );
    selector.select_chain().expect("a best route exists");
    let routes = selector.suggest_routes();
    assert_eq!(routes.len(), 3);
    assert!(routes.iter().all(|(_, s)| *s > 0.0));

    // 2. Decompose the intent into sequential legs.
    let plan = IntentDecomposer::new().decompose(swap_bridge_stake_intent());
    assert_eq!(plan.legs.len(), 3);

    // 3. Adapt the plan into a durable workflow (no HITL for the happy path).
    let adapter = PlanWorkflow::from_plan_with_options(&plan, &fast_options(false));

    let tmp = tempfile::tempdir().expect("tempdir");
    let mut engine = WorkflowEngine::open(tmp.path()).expect("open engine");
    register_std_activities(&mut engine);

    // 4. Run to completion.
    let wf_id = engine
        .start(&adapter, Some("agent-001".to_string()))
        .await
        .expect("start");
    let state = wait_terminal(&engine, &wf_id).await;
    assert!(matches!(state.status, WorkflowStatus::Completed));

    // 5. Reconcile the workflow onto the plan.
    let mut plan = plan;
    sync_plan_from_workflow(&mut plan, &adapter, &state);
    assert_eq!(plan.status, PlanStatus::Completed);
    assert!(plan.is_complete());
    assert!(plan.legs.iter().all(|l| l.status == LegStatus::Completed));
}

#[tokio::test]
async fn approval_step_pauses_until_human_signal() {
    let plan = IntentDecomposer::new().decompose(swap_bridge_stake_intent());
    let adapter = PlanWorkflow::from_plan_with_options(&plan, &fast_options(true));

    let tmp = tempfile::tempdir().expect("tempdir");
    let mut engine = WorkflowEngine::open(tmp.path()).expect("open engine");
    register_std_activities(&mut engine);

    let wf_id = engine
        .start(&adapter, Some("agent-001".to_string()))
        .await
        .expect("start");

    // The first approve step must hold the workflow.
    let paused = wait_until(&engine, &wf_id, |s| matches!(s, WorkflowStatus::Paused)).await;
    assert!(
        paused
            .step_states
            .iter()
            .any(|s| { s.step_id.ends_with("-approve") && s.status == StepStatus::Paused })
    );

    // Every leg requests its own approval; approve them one by one until the
    // workflow reaches a terminal state.
    let mut state = paused;
    for _ in 0..3 {
        if matches!(state.status, WorkflowStatus::Paused) {
            engine
                .resume(&wf_id, Signal::Approve)
                .await
                .expect("approve");
        }
        state = wait_until(&engine, &wf_id, |s| {
            matches!(s, WorkflowStatus::Paused | WorkflowStatus::Completed)
        })
        .await;
        if matches!(state.status, WorkflowStatus::Completed) {
            break;
        }
    }
    assert!(matches!(state.status, WorkflowStatus::Completed));

    let mut plan = plan;
    sync_plan_from_workflow(&mut plan, &adapter, &state);
    assert!(plan.is_complete());
}

/// The `settle` activity that fails on Base (as if the bridge reverted).
struct SettleFailOnBase;

#[async_trait]
impl Activity for SettleFailOnBase {
    fn name(&self) -> &str {
        "settle"
    }

    async fn execute(
        &self,
        input: serde_json::Value,
        _ctx: ActivityContext,
    ) -> Result<serde_json::Value, WorkflowError> {
        let chain = input
            .pointer("/action/chain")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if chain == "Base" {
            return Err(WorkflowError::ActivityError(
                "bridge settlement reverted on Base".into(),
            ));
        }
        Ok(serde_json::json!({ "status": "settled", "chain": chain }))
    }
}

#[tokio::test]
async fn failed_leg_is_compensated_in_reverse_order() {
    let plan = IntentDecomposer::new().decompose(swap_bridge_stake_intent());
    let leg0 = plan.legs[0].id; // Ethereum swap - will complete

    let adapter = PlanWorkflow::from_plan_with_options(&plan, &fast_options(false));

    let tmp = tempfile::tempdir().expect("tempdir");
    let mut engine = WorkflowEngine::open(tmp.path()).expect("open engine");
    engine.register_activity(SimulateActivity);
    engine.register_activity(SimulateEvmActivity);
    engine.register_activity(SettleFailOnBase); // overrides the happy settle

    let wf_id = engine
        .start(&adapter, Some("agent-001".to_string()))
        .await
        .expect("start");
    let state = wait_terminal(&engine, &wf_id).await;
    assert!(matches!(state.status, WorkflowStatus::Failed(_)));

    let mut plan = plan;
    sync_plan_from_workflow(&mut plan, &adapter, &state);
    assert_eq!(plan.legs[0].status, LegStatus::Completed); // swap settled
    assert_eq!(plan.legs[1].status, LegStatus::Failed); // bridge failed
    assert_eq!(plan.legs[2].status, LegStatus::Pending); // never ran
    assert_eq!(plan.status, PlanStatus::Failed);

    // The AtomicityManager sees that compensation is required.
    let mut manager = AtomicityManager::new();
    assert!(manager.should_compensate(&plan));
    let to_compensate = manager.begin_compensation(&mut plan);
    assert_eq!(to_compensate, vec![leg0]);
    assert!(plan.legs[0].status == LegStatus::Compensating);

    // The compensation workflow unwinds the completed leg.
    engine.register_activity(CompensateActivity);
    let comp_wf = CompensationWorkflow::from_plan_with_options(&plan, &fast_options(false));
    assert_eq!(
        comp_wf.step_to_leg().get(&comp_wf.steps()[0].id),
        Some(&leg0)
    );
    let comp_id = engine
        .start(&comp_wf, Some("agent-001".to_string()))
        .await
        .expect("start compensation");
    let comp_state = wait_terminal(&engine, &comp_id).await;
    assert!(matches!(comp_state.status, WorkflowStatus::Completed));

    // Reconcile: leg0 is compensated and the plan stays terminal (Failed).
    sync_compensation_from_workflow(&mut plan, &mut manager, comp_wf.step_to_leg(), &comp_state);
    assert_eq!(plan.legs[0].status, LegStatus::Compensated);
    assert_eq!(manager.compensated, vec![leg0]);
    assert!(manager.in_progress.is_empty());
    assert_eq!(plan.status, PlanStatus::Failed);
}
