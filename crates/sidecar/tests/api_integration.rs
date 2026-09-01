//! Chain-agnostic sidecar API tests: homepage discovery, policy management, and
//! EVM + Solana simulation routing. Simulation paths are network-opt-in (per-chain
//! RPC env vars), so the unconfigured-path 503 behavior is what's exercised here.

use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use std::collections::HashMap;
use std::sync::Arc;
use tempfile::TempDir;
use tower::ServiceExt;

use bastion_sidecar::{
    build_app, grond_oracle::GrondOracle, logger::AuditLogger, policy::Policy,
    simulation_evm::EvmSimulator,
};

fn test_app() -> (axum::Router, TempDir) {
    let tmp_dir = tempfile::tempdir().expect("temp dir");
    let db_path = tmp_dir.path().join("audit.sled");
    let logger = Arc::new(AuditLogger::new(db_path.to_str().expect("db path")).expect("logger"));
    let agent_store_path = tmp_dir.path().join("agents.sled");
    (
        build_app(
            Policy::default(),
            logger,
            GrondOracle::disabled(),
            Arc::new(HashMap::new()),
            agent_store_path.to_str().expect("agent store path"),
        ),
        tmp_dir,
    )
}

fn test_app_with_evm(chains: &[(&str, &str)]) -> (axum::Router, TempDir) {
    let tmp_dir = tempfile::tempdir().expect("temp dir");
    let db_path = tmp_dir.path().join("audit.sled");
    let logger = Arc::new(AuditLogger::new(db_path.to_str().expect("db path")).expect("logger"));
    let mut evm_simulators = HashMap::new();
    for (chain, url) in chains {
        evm_simulators.insert(
            chain.to_string(),
            Arc::new(EvmSimulator::for_chain(*chain, *url)),
        );
    }
    let agent_store_path = tmp_dir.path().join("agents.sled");
    (
        build_app(
            Policy::default(),
            logger,
            GrondOracle::disabled(),
            Arc::new(evm_simulators),
            agent_store_path.to_str().expect("agent store path"),
        ),
        tmp_dir,
    )
}

fn json_request(path: &str, payload: serde_json::Value) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(path)
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .expect("request")
}

fn json_request_with_method(method: &str, path: &str, payload: serde_json::Value) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(path)
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .expect("request")
}

#[tokio::test]
async fn homepage_serves_html_with_discovery_link_header() {
    let (app, _tmp_dir) = test_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::OK);

    // RFC 8288 Link header advertising the agent-discovery resources (checked
    // by isitagentready.com on the homepage).
    let link = response
        .headers()
        .get("link")
        .expect("link header present")
        .to_str()
        .expect("link header is valid UTF-8");
    assert!(link.contains("rel=\"api-catalog\""));
    assert!(link.contains("/.well-known/oauth-authorization-server"));
    assert!(link.contains("/webmcp.js"));

    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body bytes");
    let body = String::from_utf8(body.to_vec()).expect("body is valid UTF-8");
    // WebMCP detection needs an HTML homepage that loads the loader script.
    assert!(body.contains("<script src=\"/webmcp.js\"></script>"));
}

#[tokio::test]
async fn policy_full_endpoint_round_trips_allowlist() {
    let (app, _tmp_dir) = test_app();

    let put = app
        .clone()
        .oneshot(json_request_with_method(
            "PUT",
            "/policy/full",
            serde_json::json!({ "allowed_programs": ["0xabc", "0xdef"] }),
        ))
        .await
        .expect("response");
    assert_eq!(put.status(), StatusCode::OK);

    let get = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/policy")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(get.status(), StatusCode::OK);
    let body = to_bytes(get.into_body(), usize::MAX).await.expect("body");
    let payload: serde_json::Value = serde_json::from_slice(&body).expect("json");
    assert_eq!(
        payload["allowed_programs"],
        serde_json::json!(["0xabc", "0xdef"])
    );
}

fn evm_sim_payload(chain: &str) -> serde_json::Value {
    serde_json::json!({
        "transaction": { "from": "0x1111111111111111111111111111111111111111",
                          "to":   "0x2222222222222222222222222222222222222222" },
        "chain": chain,
    })
}

#[tokio::test]
async fn simulate_evm_unconfigured_chain_returns_503_with_clear_message() {
    // No EVM simulators configured at all.
    let (app, _tmp) = test_app_with_evm(&[]);

    let response = app
        .oneshot(json_request(
            "/api/v2/simulate-evm",
            evm_sim_payload("ethereum"),
        ))
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body");
    let body = String::from_utf8(body.to_vec()).expect("utf8");
    assert!(body.contains("ethereum"), "message names the chain: {body}");
    assert!(
        body.contains("ETH_RPC_URL"),
        "message names the env var: {body}"
    );
}

#[tokio::test]
async fn simulate_evm_defaults_to_celo_when_chain_omitted() {
    let (app, _tmp) = test_app_with_evm(&[]);

    let response = app
        .oneshot(json_request(
            "/api/v2/simulate-evm",
            serde_json::json!({
                "transaction": { "from": "0x1111111111111111111111111111111111111111",
                                 "to":   "0x2222222222222222222222222222222222222222" }
            }),
        ))
        .await
        .expect("response");

    // Default chain is "celo"; with nothing configured it 503s naming celo.
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body");
    let body = String::from_utf8(body.to_vec()).expect("utf8");
    assert!(body.contains("celo"), "defaults to celo: {body}");
    assert!(body.contains("CELO_RPC_URL"), "names celo env var: {body}");
}

#[tokio::test]
async fn simulate_solana_unconfigured_returns_503_with_enable_hint() {
    let (app, _tmp) = test_app();

    let response = app
        .oneshot(json_request(
            "/api/v2/simulate-solana",
            serde_json::json!({
                "to": "11111111111111111111111111111111",
                "amount": 1000,
            }),
        ))
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body");
    let body = String::from_utf8(body.to_vec()).expect("utf8");
    assert!(
        body.contains("SOLANA_RPC_URL"),
        "message names the env var: {body}"
    );
}

// Note: the positive routing path (a configured chain actually reaching its
// simulator) is a network-integration concern - `EvmSimulator` owns a blocking
// reqwest client that can't be constructed/dropped cleanly inside a test runtime.
// The 503 tests above already prove the handler looks up by the *requested* chain
// (empty map -> 503 naming that chain) and defaults correctly, which is the
// behavior change; `test_evm_rpc_env_var_mapping` covers the env-var naming.

fn empty_get(path: &str) -> Request<Body> {
    Request::builder()
        .method("GET")
        .uri(path)
        .body(Body::empty())
        .expect("request")
}

fn empty_delete(path: &str) -> Request<Body> {
    Request::builder()
        .method("DELETE")
        .uri(path)
        .body(Body::empty())
        .expect("request")
}

async fn json_body(response: axum::response::Response) -> serde_json::Value {
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body bytes");
    serde_json::from_slice(&body).expect("json body")
}

async fn register_test_agent(app: &axum::Router, did: &str) {
    let response = app
        .clone()
        .oneshot(json_request(
            "/agents",
            serde_json::json!({ "did": did, "authority_pubkey": did, "sidecar_endpoint": null }),
        ))
        .await
        .expect("response");
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn delete_agent_removes_it_and_repeat_delete_404s() {
    let (app, _tmp) = test_app();
    let did = "did:bastion:evm:0xdelete-me";
    register_test_agent(&app, did).await;

    let path = format!("/agents/{did}");
    let get_before = app.clone().oneshot(empty_get(&path)).await.expect("resp");
    assert_eq!(get_before.status(), StatusCode::OK);

    let delete = app
        .clone()
        .oneshot(empty_delete(&path))
        .await
        .expect("resp");
    assert_eq!(delete.status(), StatusCode::OK);
    let body = json_body(delete).await;
    assert_eq!(body["status"], "deregistered");

    let get_after = app.clone().oneshot(empty_get(&path)).await.expect("resp");
    assert_eq!(get_after.status(), StatusCode::NOT_FOUND);

    let delete_again = app.oneshot(empty_delete(&path)).await.expect("resp");
    assert_eq!(delete_again.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn delete_unknown_agent_returns_404() {
    let (app, _tmp) = test_app();
    let response = app
        .oneshot(empty_delete("/agents/did:bastion:evm:0xnever-registered"))
        .await
        .expect("response");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn revoke_delegation_actually_persists_not_just_a_canned_response() {
    let (app, _tmp) = test_app();
    let parent = "did:bastion:evm:0xparent";
    let child = "did:bastion:evm:0xchild";
    register_test_agent(&app, parent).await;

    let delegate = app
        .clone()
        .oneshot(json_request(
            &format!("/agents/{parent}/delegate"),
            serde_json::json!({ "child_did": child, "child_name": "child-agent" }),
        ))
        .await
        .expect("response");
    assert_eq!(delegate.status(), StatusCode::OK);

    // Confirm the delegation actually took (parent/child linked both ways).
    let children = json_body(
        app.clone()
            .oneshot(empty_get(&format!("/agents/{parent}/children")))
            .await
            .expect("response"),
    )
    .await;
    assert_eq!(children["child_count"], 1);

    let child_before = json_body(
        app.clone()
            .oneshot(empty_get(&format!("/agents/{child}")))
            .await
            .expect("response"),
    )
    .await;
    assert_eq!(child_before["parent_did"], parent);

    let revoke = app
        .clone()
        .oneshot(empty_delete(&format!(
            "/agents/{parent}/delegation/{child}"
        )))
        .await
        .expect("response");
    assert_eq!(revoke.status(), StatusCode::OK);

    // The bug this guards against: the old handler returned 200
    // "delegation_revoked" without touching storage at all. Verify the
    // relationship is actually gone on both sides.
    let children_after = json_body(
        app.clone()
            .oneshot(empty_get(&format!("/agents/{parent}/children")))
            .await
            .expect("response"),
    )
    .await;
    assert_eq!(children_after["child_count"], 0);

    let child_after = json_body(
        app.oneshot(empty_get(&format!("/agents/{child}")))
            .await
            .expect("response"),
    )
    .await;
    assert_eq!(child_after["parent_did"], serde_json::Value::Null);
}

#[tokio::test]
async fn revoke_unknown_delegation_returns_404() {
    let (app, _tmp) = test_app();
    let parent = "did:bastion:evm:0xparent-only";
    register_test_agent(&app, parent).await;

    let response = app
        .oneshot(empty_delete(&format!(
            "/agents/{parent}/delegation/did:bastion:evm:0xnever-delegated"
        )))
        .await
        .expect("response");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn exceptions_crud_round_trips_through_real_storage() {
    let (app, _tmp) = test_app();

    let list_empty = json_body(
        app.clone()
            .oneshot(empty_get("/exceptions"))
            .await
            .expect("response"),
    )
    .await;
    assert_eq!(list_empty, serde_json::json!([]));

    let create = app
        .clone()
        .oneshot(json_request(
            "/exceptions",
            serde_json::json!({
                "policy_name": "treasury-guard",
                "reason": "quarterly audit window",
                "expires_in_hours": 4,
                "approved_by": "test-suite",
            }),
        ))
        .await
        .expect("response");
    assert_eq!(create.status(), StatusCode::OK);
    let created = json_body(create).await;
    assert_eq!(created["policy_name"], "treasury-guard");
    let id = created["id"].as_str().expect("id is a string").to_string();

    // The bug this guards against: the old handler always returned `[]`
    // regardless of what was created.
    let list_after_create = json_body(
        app.clone()
            .oneshot(empty_get("/exceptions"))
            .await
            .expect("response"),
    )
    .await;
    assert_eq!(list_after_create.as_array().expect("array").len(), 1);
    assert_eq!(list_after_create[0]["id"], id);

    let delete = app
        .clone()
        .oneshot(empty_delete(&format!("/exceptions/{id}")))
        .await
        .expect("response");
    assert_eq!(delete.status(), StatusCode::OK);

    let list_after_delete = json_body(
        app.clone()
            .oneshot(empty_get("/exceptions"))
            .await
            .expect("response"),
    )
    .await;
    assert_eq!(list_after_delete, serde_json::json!([]));

    let delete_again = app
        .oneshot(empty_delete(&format!("/exceptions/{id}")))
        .await
        .expect("response");
    assert_eq!(delete_again.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn trust_policy_get_serializes_match_key_without_raw_identifier_prefix() {
    let (app, _tmp) = test_app();

    let yaml = "apiVersion: bastion.io/v1\nkind: TrustPolicy\nmetadata:\n  name: match-key-test\nspec:\n  match:\n    intent: transfer\n";
    let create = app
        .clone()
        .oneshot(json_request(
            "/policies",
            serde_json::json!({ "yaml": yaml }),
        ))
        .await
        .expect("response");
    assert_eq!(create.status(), StatusCode::OK);

    let get = app
        .oneshot(empty_get("/policies/match-key-test"))
        .await
        .expect("response");
    assert_eq!(get.status(), StatusCode::OK);
    let body = json_body(get).await;

    // The bug this guards against: the handler hardcoded the JSON key as the
    // literal Rust raw-identifier text `"r#match"` instead of `"match"`.
    assert!(
        body.get("r#match").is_none(),
        "response must not leak the raw-identifier prefix: {body}"
    );
    assert_eq!(body["match"]["intent"], "transfer");
}
