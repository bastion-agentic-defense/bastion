//! Background scanner tests: on-demand scans and stored results over the REST API.
//! Solana-coupled scan paths (expired-approval detection, which depended on the
//! retired `/simulate` handler populating pending holds) were removed with the
//! full-EVM pivot.

use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use std::sync::Arc;
use tempfile::TempDir;
use tower::ServiceExt;

use bastion_sidecar::{build_app, grond_oracle::GrondOracle, logger::AuditLogger, policy::Policy};

fn test_app(policy: Policy) -> (axum::Router, TempDir) {
    let tmp_dir = tempfile::tempdir().expect("temp dir");
    let db_path = tmp_dir.path().join("audit.sled");
    let logger = Arc::new(AuditLogger::new(db_path.to_str().expect("db path")).expect("logger"));
    let agent_store_path = tmp_dir.path().join("agents.sled");
    (
        build_app(
            policy,
            logger,
            GrondOracle::disabled(),
            Arc::new(std::collections::HashMap::new()),
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

#[tokio::test]
async fn scan_on_fresh_instance_reports_no_violations() {
    let (app, _tmp) = test_app(Policy::default());

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
async fn enforced_policy_that_the_engine_no_longer_meets_is_flagged_as_drift() {
    let (app, _tmp) = test_app(Policy::default());

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
