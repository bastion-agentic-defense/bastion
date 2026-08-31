use bastion_sidecar::{
    audit::AuditLogger, build_app, grond_oracle::GrondOracle, policy::Policy,
    simulation_evm::EvmSimulator,
};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::sync::Arc;
use tokio::signal;

async fn shutdown_signal() {
    let _ = signal::ctrl_c().await;
    eprintln!("shutdown signal received");
}

/// Initialize structured logging. Emits JSON logs when `BASTION_LOG_JSON=1`
/// (recommended for mainnet/Fly.io log ingestion), otherwise human-readable.
/// Verbosity is controlled by `RUST_LOG` (default `info`).
fn init_tracing() {
    use tracing_subscriber::{EnvFilter, fmt};
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let json = matches!(
        env::var("BASTION_LOG_JSON").as_deref(),
        Ok("1") | Ok("true")
    );
    if json {
        fmt()
            .with_env_filter(filter)
            .json()
            .flatten_event(true)
            .init();
    } else {
        fmt().with_env_filter(filter).init();
    }
}

#[tokio::main]
async fn main() {
    init_tracing();
    tracing::info!("bastion sidecar starting");
    let config_text = fs::read_to_string("config.toml").expect("read config.toml");
    let policy: Policy = toml::from_str(&config_text).expect("parse config.toml");

    let logger = Arc::new(AuditLogger::new("audit_logs").expect("create audit logger"));

    let grond_oracle = match env::var("GROND_API_URL") {
        Ok(url) if !url.is_empty() => {
            eprintln!("[bastion] GrondOSINT oracle enabled: {url}");
            GrondOracle::new(url, reqwest::Client::new())
        }
        _ => {
            eprintln!("[bastion] GrondOSINT oracle disabled (set GROND_API_URL to enable)");
            GrondOracle::disabled()
        }
    };

    // Per-chain EVM simulators, keyed by normalized lowercase chain name. Each is
    // enabled by its own RPC env var; absent chains yield a 503 on request rather
    // than silently routing to a different network.
    //
    // EvmSimulator wraps a `reqwest::blocking::Client`, which internally spins up
    // its own single-thread tokio runtime. Constructing more than one of those
    // directly on a `#[tokio::main]` worker thread panics ("cannot drop a runtime
    // in a context where blocking is not allowed") once a second client is built
    // while the first is still alive. Building them all inside `spawn_blocking`
    // runs the construction on the blocking thread pool instead, which is safe.
    let evm_simulators: HashMap<String, Arc<EvmSimulator>> = tokio::task::spawn_blocking(|| {
        let mut sims: HashMap<String, Arc<EvmSimulator>> = HashMap::new();
        for (chain, env_var) in bastion_sidecar::simulation_evm::EVM_CHAIN_ENV_VARS {
            if let Ok(url) = env::var(env_var)
                && !url.is_empty()
            {
                eprintln!("[bastion] EVM simulator enabled for {chain}: {url}");
                sims.insert(
                    (*chain).to_string(),
                    Arc::new(EvmSimulator::for_chain(*chain, url)),
                );
            }
        }
        sims
    })
    .await
    .expect("build EVM simulators");
    if evm_simulators.is_empty() {
        eprintln!(
            "[bastion] No EVM simulators configured (set ETH_RPC_URL / BASE_RPC_URL / CELO_RPC_URL / ZKSYNC_RPC_URL / ROBINHOOD_RPC_URL / ETH_SEPOLIA_RPC_URL to enable)"
        );
    }
    let evm_simulators = Arc::new(evm_simulators);

    let agent_store_path =
        env::var("BASTION_AGENT_STORE_PATH").unwrap_or_else(|_| "agent_store".to_string());

    let app = build_app(
        policy,
        logger,
        grond_oracle,
        evm_simulators,
        &agent_store_path,
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("bind to port 3000");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");
}
