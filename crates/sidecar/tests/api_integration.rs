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
