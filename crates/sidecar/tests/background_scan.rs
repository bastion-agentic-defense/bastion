//! Background scanner tests: on-demand scans and stored results over the REST API.

use anyhow::Result;
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use base64::Engine as _;
use solana_sdk::{
    message::Message, pubkey::Pubkey, signature::Keypair, signer::Signer, system_instruction,
    system_program, transaction::Transaction,
};
use std::sync::Arc;
use tempfile::TempDir;
use tower::ServiceExt;

use bastion_sidecar::{
    build_app,
    grond_oracle::GrondOracle,
    logger::AuditLogger,
    policy::Policy,
    program_client::OnChainClient,
    simulation::{Simulate, SimulationResult},
};

#[derive(Clone)]
struct MockSimulator {
    result: SimulationResult,
}

impl Simulate for MockSimulator {
    fn simulate_transaction(&self, _tx: &Transaction) -> Result<SimulationResult> {
        Ok(self.result.clone())
    }
}

fn ok_result() -> SimulationResult {
    SimulationResult {
        logs: vec!["simulated transaction".to_string()],
        units_consumed: Some(42_000),
        return_data: None,
        error: None,
        balance_changes: std::collections::HashMap::new(),
        simulation_hash: None,
    }
}

fn error_result() -> SimulationResult {
    SimulationResult {
        logs: vec!["simulated transaction".to_string()],
        units_consumed: Some(42_000),
        return_data: None,
        error: Some(serde_json::json!({"InstructionError": [0, {"Custom": 6001}]})),
        balance_changes: std::collections::HashMap::new(),
        simulation_hash: None,
    }
}

fn test_app(sim: SimulationResult, policy: Policy) -> (axum::Router, TempDir) {
    let tmp_dir = tempfile::tempdir().expect("temp dir");
    let db_path = tmp_dir.path().join("audit.sled");
    let logger = Arc::new(AuditLogger::new(db_path.to_str().expect("db path")).expect("logger"));
    let simulator: Arc<dyn Simulate + Send + Sync> = Arc::new(MockSimulator { result: sim });
    let agent_store_path = tmp_dir.path().join("agents.sled");
    (
        build_app(
            policy,
            simulator,
            logger,
            OnChainClient::disabled(),
            GrondOracle::disabled(),
            Arc::new(std::collections::HashMap::new()),
            None,
            agent_store_path.to_str().expect("agent store path"),
        ),
        tmp_dir,
    )
}

fn json_request(method: &str, path: &str, payload: serde_json::Value) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(path)
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .expect("request")
}

async fn response_json(
    app: axum::Router,
    request: Request<Body>,
) -> (StatusCode, serde_json::Value) {
    let response = app.oneshot(request).await.expect("response");
    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body bytes");
    match serde_json::from_slice(&body) {
        Ok(v) => (status, v),
        Err(e) => panic!(
            "non-JSON response (status {status}): {:?} ({e})",
            String::from_utf8_lossy(&body)
        ),
    }
}

fn simulate_payload() -> serde_json::Value {
    let payer = Keypair::new();
    let to = Pubkey::new_unique();
    let ix = system_instruction::transfer(&payer.pubkey(), &to, 1_000_000);
    let msg = Message::new(&[ix], Some(&payer.pubkey()));
    let tx = Transaction::new_unsigned(msg);
    let encoded =
        base64::engine::general_purpose::STANDARD.encode(bincode::serialize(&tx).unwrap());
    serde_json::json!({ "transaction": encoded, "intent": "scan test" })
}

#[tokio::test]
async fn scan_on_fresh_instance_reports_no_violations() {
    let (app, _tmp) = test_app(ok_result(), Policy::default());

    let (status, body) = response_json(
        app.clone(),
        json_request("POST", "/scans", serde_json::json!({})),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["expiredApprovals"], 0);
    assert_eq!(body["expiredDelegations"], 0);
    assert_eq!(body["policyDrifts"], 0);
    assert_eq!(body["unsettledTransactions"], 0);
    assert_eq!(body["findings"], serde_json::json!([]));
    assert!(body["timestamp"].as_u64().unwrap() > 0);

    // The scan is stored for later retrieval.
    let (status, stored) = response_json(
        app,
        Request::builder()
            .uri("/scan/results")
            .body(Body::empty())
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(stored["last_scan"]["timestamp"], body["timestamp"]);
}

#[tokio::test]
async fn expired_approval_is_flagged_once_past_the_ttl() {
    let policy = Policy {
        allowed_programs: vec![system_program::id().to_string()],
        simulation_checks_enabled: true,
        ..Policy::default()
    };
    let (app, _tmp) = test_app(error_result(), policy);

    // A failing simulation holds the transaction for human approval.
    let (status, body) = response_json(
        app.clone(),
        json_request("POST", "/simulate", simulate_payload()),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    let block_id = body["block_id"].as_str().expect("block_id").to_string();

    // A zero TTL makes every open hold count as expired. SAFETY: no other test
    // in this binary reads this variable, and scans only run on demand here.
    unsafe { std::env::set_var("BASTION_SCAN_APPROVAL_TTL_SECS", "0") };
    let (status, body) =
        response_json(app, json_request("POST", "/scans", serde_json::json!({}))).await;
    unsafe { std::env::remove_var("BASTION_SCAN_APPROVAL_TTL_SECS") };

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["expiredApprovals"], 1);
    let finding = &body["findings"][0];
    assert_eq!(finding["kind"], "expired_approval");
    assert_eq!(finding["id"], block_id);
}

#[tokio::test]
async fn enforced_policy_that_the_engine_no_longer_meets_is_flagged_as_drift() {
    let (app, _tmp) = test_app(ok_result(), Policy::default());

    let yaml = r#"
apiVersion: bastion.io/v1
kind: TrustPolicy
metadata:
  name: strict-ops
spec:
  validate:
    blocklist: ["bad-actor"]
"#;
    let (status, _) = response_json(
        app.clone(),
        json_request("POST", "/policies", serde_json::json!({ "yaml": yaml })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    let (status, _) = response_json(
        app.clone(),
        json_request(
            "POST",
            "/policies/strict-ops/mode",
            serde_json::json!({ "mode": "enforce" }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    // The live engine has an empty blocklist, so the enforced policy has drifted.
    let (status, body) =
        response_json(app, json_request("POST", "/scans", serde_json::json!({}))).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["policyDrifts"], 1);
    let finding = &body["findings"][0];
    assert_eq!(finding["kind"], "policy_drift");
    assert_eq!(finding["id"], "strict-ops");
}
