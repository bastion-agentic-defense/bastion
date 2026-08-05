use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use bastion_workflow::{
    WorkflowEngine, Signal,
    activities::{
        simulate::SimulateActivity,
        simulate_evm::SimulateEvmActivity,
        approve::ApproveActivity,
        settle::SettleActivity,
        http_call::HttpCallActivity,
        sleep::SleepActivity,
        fetch_secret::FetchSecretActivity,
    },
};
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

pub fn build_workflow_engine() -> Arc<WorkflowEngine> {
    let data_dir = std::env::var(String::from("BASTION_WORKFLOW_DIR"))
        .unwrap_or_else(|_| String::from("/tmp/bastion-workflows"));
    let path = std::path::Path::new(&data_dir);
    std::fs::create_dir_all(path).ok();

    let mut engine = WorkflowEngine::open(path)
        .expect("failed to open workflow store");

    engine.register_activity(SimulateActivity);
    engine.register_activity(SimulateEvmActivity);
    engine.register_activity(ApproveActivity);
    engine.register_activity(SettleActivity);
    engine.register_activity(HttpCallActivity);
    engine.register_activity(SleepActivity);
    engine.register_activity(FetchSecretActivity);

    Arc::new(engine)
}

pub async fn start_workflow(
    State(state): State<AppState>,
    Json(req): Json<StartWorkflowRequest>,
) -> impl IntoResponse {
    let engine = &state.workflow_engine;
    match engine.start(&StubDefinition(req.definition), req.agent_id).await {
        Ok(id) => (StatusCode::CREATED, Json(serde_json::json!({ "workflow_id": id, "status": "running" }))).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: e.to_string() })).into_response(),
    }
}

pub async fn start_workflow_yaml(
    State(state): State<AppState>,
    Json(req): Json<StartYamlRequest>,
) -> impl IntoResponse {
    let engine = &state.workflow_engine;
    match engine.start_yaml(&req.yaml, req.agent_id).await {
        Ok(id) => (StatusCode::CREATED, Json(serde_json::json!({ "workflow_id": id, "status": "running" }))).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: e.to_string() })).into_response(),
    }
}

pub async fn list_workflows(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let engine = &state.workflow_engine;
    match engine.list() {
        Ok(states) => Json(states).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })).into_response(),
    }
}

pub async fn get_workflow(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let engine = &state.workflow_engine;
    match engine.state(&id) {
        Ok(state) => Json(state).into_response(),
        Err(e) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: e.to_string() })).into_response(),
    }
}

pub async fn get_workflow_events(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let engine = &state.workflow_engine;
    match engine.replay(&id) {
        Ok(events) => Json(events).into_response(),
        Err(e) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: e.to_string() })).into_response(),
    }
}

pub async fn signal_workflow(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<SignalRequest>,
) -> impl IntoResponse {
    let engine = &state.workflow_engine;
    let signal = match req.signal.as_str() {
        "approve" => Signal::Approve,
        "cancel" => Signal::Cancel,
        _ => return (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: format!("unknown signal: {}", req.signal) })).into_response(),
    };
    match engine.resume(&id, signal).await {
        Ok(()) => Json(serde_json::json!({ "ok": true })).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: e.to_string() })).into_response(),
    }
}

pub async fn delete_workflow(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let engine = &state.workflow_engine;
    match engine.cancel(&id) {
        Ok(()) => Json(serde_json::json!({ "ok": true, "status": "cancelled" })).into_response(),
        Err(e) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: e.to_string() })).into_response(),
    }
}

struct StubDefinition(String);

#[async_trait::async_trait]
impl bastion_workflow::WorkflowDefinition for StubDefinition {
    fn name(&self) -> &str { &self.0 }
    fn steps(&self) -> Vec<bastion_workflow::WorkflowStep> { vec![] }
}
