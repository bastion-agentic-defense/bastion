use axum::{Json, extract::{Path, Query, State}};
use bastion_workflow::Signal;
use serde::Deserialize;
use std::sync::Arc;

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct StartWorkflowRequest {
    pub yaml: String,
    pub agent_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SignalRequest {
    pub signal: String,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub agent_id: Option<String>,
}

pub async fn start_workflow_yaml(
    State(state): State<AppState>,
    Json(req): Json<StartWorkflowRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let wf_id = state.workflow_engine.start_yaml(&req.yaml, req.agent_id)
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(Json(serde_json::json!({ "workflow_id": wf_id })))
}

pub async fn get_workflow(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let wf = state.workflow_engine.state(&id)
        .map_err(|e| (axum::http::StatusCode::NOT_FOUND, e.to_string()))?;
    Ok(Json(serde_json::to_value(&wf).map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?))
}

pub async fn get_workflow_events(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let events = state.workflow_engine.replay(&id)
        .map_err(|e| (axum::http::StatusCode::NOT_FOUND, e.to_string()))?;
    Ok(Json(serde_json::to_value(&events).map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?))
}

pub async fn signal_workflow(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<SignalRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let signal = match req.signal.as_str() {
        "approve" => Signal::Approve,
        "cancel" => Signal::Cancel,
        _ => return Err((axum::http::StatusCode::BAD_REQUEST, "signal must be 'approve' or 'cancel'".into())),
    };
    state.workflow_engine.resume(&id, signal)
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(Json(serde_json::json!({ "status": "signaled" })))
}

pub async fn cancel_workflow(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    state.workflow_engine.cancel(&id)
        .map_err(|e| (axum::http::StatusCode::NOT_FOUND, e.to_string()))?;
    Ok(Json(serde_json::json!({ "status": "cancelled" })))
}

pub async fn list_workflows(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let workflows = if let Some(agent_id) = query.agent_id {
        state.workflow_engine.list_by_agent(&agent_id)
    } else {
        state.workflow_engine.list()
    }
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(serde_json::to_value(&workflows).map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?))
}