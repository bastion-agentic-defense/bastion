use axum::{
    Json,
    extract::{Path, State},
};
use bastion_policy_engine::TrustPolicy;
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePolicyRequest {
    pub yaml: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolicyModeRequest {
    pub mode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestPolicyRequest {
    pub intent: String,
    pub chain: String,
    pub amount: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestPolicyResponse {
    pub matches: bool,
    pub rules_evaluated: usize,
    pub decision: String,
}

pub(crate) async fn list_trust_policies(
    State(state): State<AppState>,
) -> Json<Vec<serde_json::Value>> {
    let policies = state.policy_stores.read().await;
    let result: Vec<_> = policies
        .iter()
        .map(|p| {
            serde_json::json!({
                "name": p.policy.metadata.name,
                "api_version": p.policy.api_version,
                "kind": p.policy.kind,
                "mode": format!("{:?}", p.current_mode),
                "evaluations": p.status().evaluations,
                "blocks": p.status().blocks,
            })
        })
        .collect();
    Json(result)
}

pub(crate) async fn create_trust_policy(
    State(state): State<AppState>,
    Json(req): Json<CreatePolicyRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let policy = TrustPolicy::from_yaml(&req.yaml).map_err(|e| {
        (
            axum::http::StatusCode::BAD_REQUEST,
            format!("Invalid YAML: {}", e),
        )
    })?;

    let name = policy.metadata.name.clone();
    let lifecycle = bastion_policy_engine::lifecycle::PolicyLifecycle::new(policy);
    state.policy_stores.write().await.push(lifecycle);

    Ok(Json(serde_json::json!({
        "status": "created",
        "name": name,
        "mode": "audit"
    })))
}

pub(crate) async fn get_trust_policy(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let policies = state.policy_stores.read().await;
    let p = policies
        .iter()
        .find(|p| p.policy.metadata.name == name)
        .ok_or_else(|| (axum::http::StatusCode::NOT_FOUND, "Policy not found".into()))?;

    Ok(Json(serde_json::json!({
        "name": p.policy.metadata.name,
        "apiVersion": p.policy.api_version,
        "kind": p.policy.kind,
        "mode": format!("{:?}", p.current_mode),
        "r#match": p.policy.spec.r#match,
        "validate": p.policy.spec.validate,
        "mutate": p.policy.spec.mutate,
        "exceptions": p.policy.spec.exceptions,
        "status": p.status(),
    })))
}

pub(crate) async fn update_trust_policy(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Json(req): Json<CreatePolicyRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let new_policy = TrustPolicy::from_yaml(&req.yaml).map_err(|e| {
        (
            axum::http::StatusCode::BAD_REQUEST,
            format!("Invalid YAML: {}", e),
        )
    })?;

    let mut policies = state.policy_stores.write().await;
    let pos = policies
        .iter()
        .position(|p| p.policy.metadata.name == name)
        .ok_or_else(|| (axum::http::StatusCode::NOT_FOUND, "Policy not found".into()))?;

    let lifecycle = bastion_policy_engine::lifecycle::PolicyLifecycle::new(new_policy);
    policies[pos] = lifecycle;

    Ok(Json(
        serde_json::json!({ "status": "updated", "name": name }),
    ))
}

pub(crate) async fn delete_trust_policy(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let mut policies = state.policy_stores.write().await;
    let pos = policies
        .iter()
        .position(|p| p.policy.metadata.name == name)
        .ok_or_else(|| (axum::http::StatusCode::NOT_FOUND, "Policy not found".into()))?;

    policies.remove(pos);
    Ok(Json(
        serde_json::json!({ "status": "deleted", "name": name }),
    ))
}

pub(crate) async fn validate_trust_policy(
    Json(req): Json<CreatePolicyRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    match TrustPolicy::from_yaml(&req.yaml) {
        Ok(policy) => Ok(Json(serde_json::json!({
            "valid": true,
            "name": policy.metadata.name,
            "rule_count": policy.to_policy_rules().len(),
        }))),
        Err(e) => Ok(Json(serde_json::json!({
            "valid": false,
            "error": e.to_string(),
        }))),
    }
}

pub(crate) async fn test_trust_policy(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Json(req): Json<TestPolicyRequest>,
) -> Result<Json<TestPolicyResponse>, (axum::http::StatusCode, String)> {
    let policies = state.policy_stores.read().await;
    let policy = policies
        .iter()
        .find(|p| p.policy.metadata.name == name)
        .ok_or_else(|| (axum::http::StatusCode::NOT_FOUND, "Policy not found".into()))?;

    let matches = policy.policy.matches(&req.intent, &req.chain);
    let rules = policy.policy.to_policy_rules();

    let decision = if !matches { "no_match" } else { "would_audit" };

    Ok(Json(TestPolicyResponse {
        matches,
        rules_evaluated: rules.len(),
        decision: decision.to_string(),
    }))
}

pub(crate) async fn set_policy_mode(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Json(req): Json<PolicyModeRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let mut policies = state.policy_stores.write().await;
    let policy = policies
        .iter_mut()
        .find(|p| p.policy.metadata.name == name)
        .ok_or_else(|| (axum::http::StatusCode::NOT_FOUND, "Policy not found".into()))?;

    match req.mode.as_str() {
        "enforce" => policy.enforce(),
        "audit" => policy.audit(),
        _ => {
            return Err((
                axum::http::StatusCode::BAD_REQUEST,
                "Mode must be 'audit' or 'enforce'".into(),
            ));
        }
    }

    Ok(Json(
        serde_json::json!({ "status": "updated", "name": name, "mode": req.mode }),
    ))
}

pub(crate) async fn get_policy_report(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let policies = state.policy_stores.read().await;
    let p = policies
        .iter()
        .find(|p| p.policy.metadata.name == name)
        .ok_or_else(|| (axum::http::StatusCode::NOT_FOUND, "Policy not found".into()))?;

    Ok(Json(serde_json::json!({
        "policy_name": p.policy.metadata.name,
        "mode": format!("{:?}", p.current_mode),
        "evaluations": 0,
        "blocks": 0,
        "compliant": true,
    })))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateExceptionRequest {
    pub policy_name: String,
    pub reason: String,
    pub expires_in_hours: u64,
    pub approved_by: String,
}

pub(crate) async fn list_exceptions() -> Json<Vec<serde_json::Value>> {
    Json(vec![])
}

pub(crate) async fn create_exception(
    Json(req): Json<CreateExceptionRequest>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "created",
        "policy_name": req.policy_name,
        "reason": req.reason,
        "expires_in_hours": req.expires_in_hours,
    }))
}

pub(crate) async fn delete_exception(Path(_id): Path<String>) -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "deleted" }))
}

pub(crate) async fn trigger_scan(State(state): State<AppState>) -> Json<serde_json::Value> {
    let result = crate::scanner::run_scan(&state).await;
    Json(serde_json::to_value(result).unwrap_or_default())
}

pub(crate) async fn get_scan_results(State(state): State<AppState>) -> Json<serde_json::Value> {
    let last = state.last_scan.read().await.clone();
    Json(serde_json::json!({ "last_scan": last }))
}
