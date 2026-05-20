#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use anyhow::{Result, bail};
use laminar::{
    config::{loader::load_config, validator::validate_config},
    health::tcp::start_health_checker,
    proxy::tcp::start_tcp_proxy,
    state::app::{AppState, SharedAppState},
};
use std::sync::Arc;
use tokio::sync::{RwLock, watch};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let path = std::env::args().nth(1).unwrap_or_else(|| "laminar_config.yaml".to_string());

    info!("loading config from {}", path);
    let config = load_config(&path)?;

    validate_config(&config)?;
    info!("config validation successful");

    let listener_host = config.server.host.clone();
    let listener_port = config.server.port;

    let health_interval = config.load_balancer.health_check_interval_secs;

    let state = AppState::build(config);
    info!("initialized {} upstream pools", state.upstreams.len());
    if state.upstreams.is_empty() {
        bail!("no upstreams configured");
    }

    let shared_state: SharedAppState = Arc::new(RwLock::new(state));

    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    let health_state = shared_state.clone();
    let health_rx = shutdown_rx.clone();
    tokio::spawn(async move {
        start_health_checker(health_state, health_interval, health_rx).await;
    });

    let listener_address = format!("{listener_host}:{listener_port}");
    let proxy_state = shared_state.clone();

    tokio::select! {
        res = start_tcp_proxy(&listener_address, proxy_state, shutdown_rx) => {
            if let Err(e) = res {
                error!("proxy error: {}", e);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            info!("shutdown signal received");
            let _ = shutdown_tx.send(true);
        }
    }

    info!("waiting for active connections to close...");
    loop {
        let conn_count = shared_state.read().await.total_connections();
        if conn_count == 0 {
            break;
        }
        info!("{} active connections remaining...", conn_count);
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }

    info!("all connections closed. goodbye!");
    Ok(())
}
