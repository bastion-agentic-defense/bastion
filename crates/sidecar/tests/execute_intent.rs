use anyhow::Result;
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
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
    simulation::{ReturnData, Simulate, SimulationResult},
};

#[derive(Clone)]
struct MockSimulator {
    result: SimulationResult,
}

impl Simulate for MockSimulator {
    fn simulate_transaction(
        &self,
        _tx: &solana_sdk::transaction::Transaction,
    ) -> Result<SimulationResult> {
        Ok(self.result.clone())
    }
}

fn mock_result() -> SimulationResult {
    SimulationResult {
        logs: vec!["simulated transaction".to_string()],
        units_consumed: Some(42_000),
        return_data: Some(ReturnData {
            data: "AQID".to_string(),
            encoding: "base64".to_string(),
            program_id: solana_sdk::system_program::id().to_string(),
        }),
        error: None,
        balance_changes: std::collections::HashMap::new(),
        simulation_hash: None,
    }
}

fn test_app() -> (axum::Router, TempDir) {
    let tmp_dir = tempfile::tempdir().expect("temp dir");
    let db_path = tmp_dir.path().join("audit.sled");
    let logger = Arc::new(AuditLogger::new(db_path.to_str().expect("db path")).expect("logger"));
    let simulator: Arc<dyn Simulate + Send + Sync> = Arc::new(MockSimulator {
        result: mock_result(),
    });
    let agent_store_path = tmp_dir.path().join("agents.sled");
    (
        build_app(
            Policy::default(),
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
    (status, serde_json::from_slice(&body).expect("body is JSON"))
}

fn sample_intent(require_approval: bool) -> serde_json::Value {
    serde_json::json!({
        "intent": {
            "description": "swap USDC for ETH then stake",
            "agent_id": "agent-001",
            "source_chain": "Ethereum",
            "actions": [
                {
                    "action_type": "Swap",
                    "chain": "Ethereum",
                    "protocol": "uniswap",
                    "token": "USDC",
                    "amount": 1000,
                    "destination": "0xabc",
                    "metadata": {}
                },
                {
                    "action_type": "Stake",
                    "chain": "Base",
                    "protocol": "aerodrome",
                    "token": "USDC",
                    "amount": 500,
                    "destination": "0xdef",
                    "metadata": {}
                }
            ],
            "constraints": []
        },
        "require_approval": require_approval
    })
}

#[tokio::test]
async fn execute_intent_decomposes_routes_and_starts_workflow() {
    let (app, _tmp) = test_app();

    let (status, body) =
        response_json(app, json_request("POST", "/execute", sample_intent(false))).await;

    assert_eq!(status, StatusCode::CREATED);
    let plan_id = body["plan_id"].as_str().expect("plan_id");
    assert_eq!(plan_id.len(), 36);
    assert_eq!(body["workflow_id"].as_str().expect("workflow_id").len(), 36);
    assert_eq!(body["status"], "running");

    // Route selection exposes a best chain plus ranked alternatives.
    assert!(body["selected_chain"].is_string());
    let routes = body["routes"].as_array().expect("routes");
    assert!(!routes.is_empty());
    assert!(routes.iter().all(|r| r["score"].as_f64().unwrap() > 0.0));

    // The intent decomposed into two legs with the declared chains.
    let legs = body["legs"].as_array().expect("legs");
    assert_eq!(legs.len(), 2);
    assert_eq!(legs[0]["chain"], "Ethereum");
    assert_eq!(legs[0]["action_type"], "Swap");
    assert_eq!(legs[1]["chain"], "Base");
    assert_eq!(legs[1]["action_type"], "Stake");
}

#[tokio::test]
async fn execute_intent_rejects_empty_actions() {
    let (app, _tmp) = test_app();
    let mut payload = sample_intent(false);
    payload["intent"]["actions"] = serde_json::json!([]);

    let (status, body) = response_json(app, json_request("POST", "/execute", payload)).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(body["error"].as_str().unwrap().contains("no actions"));
}

#[tokio::test]
async fn tracked_plan_completes_and_compensate_conflicts_when_healthy() {
    let (app, _tmp) = test_app();

    let (status, body) = response_json(
        app.clone(),
        json_request("POST", "/execute", sample_intent(false)),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    let plan_id = body["plan_id"].as_str().expect("plan_id").to_string();
    let workflow_id = body["workflow_id"]
        .as_str()
        .expect("workflow_id")
        .to_string();

    // The background workflow runs with the stub activities and should finish.
    let mut plan_body = serde_json::Value::Null;
    for _ in 0..200 {
        let (_, body) = response_json(
            app.clone(),
            Request::builder()
                .uri(format!("/execute/{plan_id}"))
                .body(Body::empty())
                .expect("request"),
        )
        .await;
        if body["workflow_status"] == "completed" {
            plan_body = body;
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(25)).await;
    }
    assert_eq!(
        plan_body["workflow_status"], "completed",
        "workflow did not complete in time"
    );
    assert_eq!(plan_body["plan"]["status"], "Completed");
    assert_eq!(plan_body["workflow_id"].as_str().unwrap(), workflow_id);
    assert!(
        plan_body["plan"]["legs"]
            .as_array()
            .unwrap()
            .iter()
            .all(|l| l["status"] == "Completed")
    );

    // Nothing failed, so there is nothing to compensate.
    let (status, body) = response_json(
        app,
        json_request(
            "POST",
            &format!("/execute/{plan_id}/compensate"),
            serde_json::json!({}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert!(body["error"].as_str().unwrap().contains("no failed legs"));
}

#[tokio::test]
async fn get_tracked_plan_unknown_id_returns_404() {
    let (app, _tmp) = test_app();
    let (status, body) = response_json(
        app,
        Request::builder()
            .uri("/execute/does-not-exist")
            .body(Body::empty())
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(body["error"].as_str().unwrap().contains("not found"));
}
