use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use bastion_core::execution::plan::{ExecutionPlan, Intent, PlanStatus};
use bastion_core::execution::{AtomicityManager, ChainMetrics, IntentDecomposer, RouteSelector};
use bastion_core::transaction::Chain;
use bastion_workflow::plan_adapter::{
    CompensationWorkflow, PlanAdapterOptions, PlanWorkflow, sync_compensation_from_workflow,
    sync_plan_from_workflow,
};
use bastion_workflow::{
    Signal, WorkflowEngine,
    activities::{
        approve::ApproveActivity, compensate::CompensateActivity,
        fetch_secret::FetchSecretActivity, http_call::HttpCallActivity, settle::SettleActivity,
        simulate::SimulateActivity, simulate_evm::SimulateEvmActivity, sleep::SleepActivity,
    },
};
use std::collections::HashMap;
use std::sync::Arc;

use crate::AppState;

#[derive(Deserialize)]
pub struct StartWorkflowRequest {
    pub definition: String,
    pub agent_id: Option<String>,
}

#[derive(Deserialize)]
pub struct StartYamlRequest {
    pub yaml: String,
    pub agent_id: Option<String>,
}

#[derive(Deserialize)]
pub struct SignalRequest {
    pub signal: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub fn build_workflow_engine(agent_store_path: &str) -> Arc<WorkflowEngine> {
    // BASTION_WORKFLOW_DIR wins when set. Otherwise the store lives next to the
    // agent store, which keeps it durable in production and isolated per test
    // (each test passes a tempdir-backed agent store path, so parallel tests
    // cannot collide on a shared sled lock).
    let data_dir = std::env::var(String::from("BASTION_WORKFLOW_DIR"))
        .unwrap_or_else(|_| format!("{agent_store_path}-workflows"));
    let path = std::path::Path::new(&data_dir);
    std::fs::create_dir_all(path).ok();

    let mut engine = WorkflowEngine::open(path).expect("failed to open workflow store");

    engine.register_activity(SimulateActivity);
    engine.register_activity(SimulateEvmActivity);
    engine.register_activity(ApproveActivity);
    engine.register_activity(SettleActivity);
    engine.register_activity(CompensateActivity);
    engine.register_activity(HttpCallActivity);
    engine.register_activity(SleepActivity);
    engine.register_activity(FetchSecretActivity);

    Arc::new(engine)
}

pub(crate) async fn start_workflow(
    State(state): State<AppState>,
    Json(req): Json<StartWorkflowRequest>,
) -> impl IntoResponse {
    let engine = &state.workflow_engine;
    match engine
        .start(&StubDefinition(req.definition), req.agent_id)
        .await
    {
        Ok(id) => (
            StatusCode::CREATED,
            Json(serde_json::json!({ "workflow_id": id, "status": "running" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
            .into_response(),
    }
}

pub(crate) async fn start_workflow_yaml(
    State(state): State<AppState>,
    Json(req): Json<StartYamlRequest>,
) -> impl IntoResponse {
    let engine = &state.workflow_engine;
    match engine.start_yaml(&req.yaml, req.agent_id).await {
        Ok(id) => (
            StatusCode::CREATED,
            Json(serde_json::json!({ "workflow_id": id, "status": "running" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
            .into_response(),
    }
}

pub(crate) async fn list_workflows(State(state): State<AppState>) -> impl IntoResponse {
    let engine = &state.workflow_engine;
    match engine.list() {
        Ok(states) => Json(states).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
            .into_response(),
    }
}

pub(crate) async fn get_workflow(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let engine = &state.workflow_engine;
    match engine.state(&id) {
        Ok(state) => Json(state).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
            .into_response(),
    }
}

pub(crate) async fn get_workflow_events(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let engine = &state.workflow_engine;
    match engine.replay(&id) {
        Ok(events) => Json(events).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
            .into_response(),
    }
}

pub(crate) async fn signal_workflow(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<SignalRequest>,
) -> impl IntoResponse {
    let engine = &state.workflow_engine;
    let signal = match req.signal.as_str() {
        "approve" => Signal::Approve,
        "cancel" => Signal::Cancel,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: format!("unknown signal: {}", req.signal),
                }),
            )
                .into_response();
        }
    };
    match engine.resume(&id, signal).await {
        Ok(()) => Json(serde_json::json!({ "ok": true })).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
            .into_response(),
    }
}

pub(crate) async fn delete_workflow(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let engine = &state.workflow_engine;
    match engine.cancel(&id) {
        Ok(()) => Json(serde_json::json!({ "ok": true, "status": "cancelled" })).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
            .into_response(),
    }
}

struct StubDefinition(String);

#[async_trait::async_trait]
impl bastion_workflow::WorkflowDefinition for StubDefinition {
    fn name(&self) -> &str {
        &self.0
    }
    fn steps(&self) -> Vec<bastion_workflow::WorkflowStep> {
        vec![]
    }
}

// ─── Settlement executor: intent -> plan -> durable workflow ────────────────

/// An intent submitted to `/execute`, plus execution preferences.
#[derive(Deserialize)]
pub struct ExecuteIntentRequest {
    pub intent: Intent,
    #[serde(default = "default_require_approval")]
    pub require_approval: bool,
}

fn default_require_approval() -> bool {
    true
}

/// A plan tracked in the sidecar for reconciliation and compensation.
#[derive(Clone)]
pub(crate) struct TrackedPlan {
    pub plan: ExecutionPlan,
    pub workflow_id: String,
    /// Planning adapter used to map workflow steps back onto legs.
    pub adapter: PlanWorkflow,
    /// Tracks compensation progress across requests for this plan.
    pub manager: AtomicityManager,
    pub compensation_workflow_id: Option<String>,
    /// Step id -> leg id mapping for the compensation workflow.
    pub comp_step_to_leg: HashMap<String, uuid::Uuid>,
}

/// A route selector pre-seeded with baseline per-chain metrics. In production
/// these come from the observability layer; the defaults keep the endpoint
/// self-contained and usable on a fresh deployment.
fn default_route_selector() -> RouteSelector {
    let mut sel = RouteSelector::new();
    let chains = [
        (
            Chain::Solana,
            ChainMetrics {
                cost_per_tx: 0.001,
                latency_ms: 400,
                reliability_score: 0.96,
                congestion_level: 0.25,
            },
        ),
        (
            Chain::Base,
            ChainMetrics {
                cost_per_tx: 0.01,
                latency_ms: 2000,
                reliability_score: 0.97,
                congestion_level: 0.15,
            },
        ),
        (
            Chain::Ethereum,
            ChainMetrics {
                cost_per_tx: 4.5,
                latency_ms: 12_000,
                reliability_score: 0.99,
                congestion_level: 0.55,
            },
        ),
        (
            Chain::Polygon,
            ChainMetrics {
                cost_per_tx: 0.005,
                latency_ms: 2200,
                reliability_score: 0.95,
                congestion_level: 0.2,
            },
        ),
        (
            Chain::Arbitrum,
            ChainMetrics {
                cost_per_tx: 0.02,
                latency_ms: 1500,
                reliability_score: 0.97,
                congestion_level: 0.2,
            },
        ),
        (
            Chain::Celo,
            ChainMetrics {
                cost_per_tx: 0.001,
                latency_ms: 5000,
                reliability_score: 0.94,
                congestion_level: 0.1,
            },
        ),
    ];
    for (chain, metrics) in chains {
        sel.add_chain(chain, metrics);
    }
    sel
}

/// Accepts an [`Intent`], decomposes it into a plan, routes it through the
/// route selector, and starts it as a durable workflow.
pub(crate) async fn execute_intent(
    State(state): State<AppState>,
    Json(req): Json<ExecuteIntentRequest>,
) -> impl IntoResponse {
    if req.intent.actions.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "intent has no actions".into(),
            }),
        )
            .into_response();
    }

    // 1. Route: score candidate chains from metrics.
    let selector = default_route_selector();
    let selected_chain = selector.select_chain();
    let routes = selector.suggest_routes();

    // 2. Decompose the intent into sequential legs and mark it validated.
    let decomposer = IntentDecomposer::new();
    let mut plan = decomposer.decompose(req.intent);
    plan.created_at = chrono::Utc::now().timestamp() as u64;
    plan.status = PlanStatus::Validated;

    // 3. Adapt the plan into a durable workflow definition.
    let options = PlanAdapterOptions {
        require_approval: req.require_approval,
        ..Default::default()
    };
    let adapter = PlanWorkflow::from_plan_with_options(&plan, &options);

    // 4. Start the workflow durably.
    let agent_id = plan.intent.agent_id.as_str().to_string();
    let workflow_id = match state.workflow_engine.start(&adapter, Some(agent_id)).await {
        Ok(id) => id,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
                .into_response();
        }
    };

    let plan_id = plan.id.to_string();
    let legs: Vec<serde_json::Value> = plan
        .legs
        .iter()
        .map(|leg| {
            serde_json::json!({
                "id": leg.id,
                "chain": leg.action.chain,
                "action_type": leg.action.action_type,
                "protocol": leg.action.protocol,
                "status": leg.status,
            })
        })
        .collect();

    state.execution_plans.write().await.insert(
        plan_id.clone(),
        TrackedPlan {
            plan,
            workflow_id: workflow_id.clone(),
            adapter,
            manager: AtomicityManager::new(),
            compensation_workflow_id: None,
            comp_step_to_leg: HashMap::new(),
        },
    );

    (
        StatusCode::CREATED,
        Json(serde_json::json!({
            "plan_id": plan_id,
            "workflow_id": workflow_id,
            "status": "running",
            "selected_chain": selected_chain,
            "routes": routes.iter().map(|(c, s)| serde_json::json!({ "chain": c, "score": s })).collect::<Vec<_>>(),
            "legs": legs,
            "require_approval": req.require_approval,
        })),
    )
        .into_response()
}

/// Returns a tracked plan with its live workflow status, reconciling the plan
/// legs from the workflow state and any in-flight compensation.
pub(crate) async fn get_tracked_plan(
    State(state): State<AppState>,
    Path(plan_id): Path<String>,
) -> impl IntoResponse {
    let mut plans = state.execution_plans.write().await;
    let Some(tracked) = plans.get_mut(&plan_id) else {
        return (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("plan not found: {plan_id}"),
            }),
        )
            .into_response();
    };

    if let Ok(wf_state) = state.workflow_engine.state(&tracked.workflow_id) {
        sync_plan_from_workflow(&mut tracked.plan, &tracked.adapter, &wf_state);
    }
    if let Some(comp_id) = tracked.compensation_workflow_id.clone()
        && let Ok(comp_state) = state.workflow_engine.state(&comp_id)
    {
        sync_compensation_from_workflow(
            &mut tracked.plan,
            &mut tracked.manager,
            &tracked.comp_step_to_leg,
            &comp_state,
        );
    }
    let wf_status = state
        .workflow_engine
        .state(&tracked.workflow_id)
        .map(|s| s.status)
        .ok();
    let comp_status = tracked.manager.compensation_status();

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "plan_id": plan_id,
            "plan": tracked.plan,
            "workflow_id": tracked.workflow_id,
            "workflow_status": wf_status,
            "compensation_workflow_id": tracked.compensation_workflow_id,
            "compensation": {
                "total_compensated": comp_status.total_compensated,
                "total_in_progress": comp_status.total_in_progress,
                "total_failed": comp_status.total_failed,
            },
        })),
    )
        .into_response()
}

/// Begins compensating a failed plan: reconciles the workflow outcome onto the
/// plan, then starts a `compensate` workflow that unwinds completed legs in
/// reverse order.
pub(crate) async fn compensate_plan(
    State(state): State<AppState>,
    Path(plan_id): Path<String>,
) -> impl IntoResponse {
    let mut plans = state.execution_plans.write().await;
    let Some(tracked) = plans.get_mut(&plan_id) else {
        return (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("plan not found: {plan_id}"),
            }),
        )
            .into_response();
    };

    // Reconcile the latest workflow outcome onto the plan.
    match state.workflow_engine.state(&tracked.workflow_id) {
        Ok(wf_state) => {
            sync_plan_from_workflow(&mut tracked.plan, &tracked.adapter, &wf_state);
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
                .into_response();
        }
    }

    let manager = AtomicityManager::new();
    if !manager.should_compensate(&tracked.plan) {
        return (
            StatusCode::CONFLICT,
            Json(serde_json::json!({
                "error": "plan has no failed legs requiring compensation",
                "plan_status": tracked.plan.status,
            })),
        )
            .into_response();
    }

    let leg_ids = tracked.manager.begin_compensation(&mut tracked.plan);

    let comp_wf = CompensationWorkflow::from_plan(&tracked.plan);
    tracked.comp_step_to_leg = comp_wf.step_to_leg().clone();
    let compensation_id = match state
        .workflow_engine
        .start(
            &comp_wf,
            Some(tracked.plan.intent.agent_id.as_str().to_string()),
        )
        .await
    {
        Ok(id) => id,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
                .into_response();
        }
    };
    tracked.compensation_workflow_id = Some(compensation_id.clone());

    let compensating_legs: Vec<String> = leg_ids.iter().map(|l| l.to_string()).collect();
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "plan_id": plan_id,
            "status": tracked.plan.status,
            "compensation_workflow_id": compensation_id,
            "compensating_legs": compensating_legs,
        })),
    )
        .into_response()
}
