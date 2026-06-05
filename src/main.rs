#![warn(clippy::all)]
#![warn(clippy::pedantic)]
mod admin;
use anyhow::{Result, bail};
use laminar::{
    config::{loader::load_config, validator::validate_config},
    health::tcp::start_health_checker,
    proxy::tcp::start_tcp_proxy,
    state::app::{AppState, SharedAppState},
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().json().with_current_span(true).with_span_list(true).init();

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

    let health_state = shared_state.clone();
    let admin_state = shared_state.clone();

    tokio::spawn(async move {
        if let Err(error) = admin::http::start_admin_server("127.0.0.1:9090", admin_state).await {
            tracing::error!("admin server failed: {:?}", error);
        }
    });
    tokio::spawn(async move {
        start_health_checker(health_state, health_interval).await;
    });

    let listener_address = format!("{listener_host}:{listener_port}");

    start_tcp_proxy(&listener_address, shared_state).await?;

    Ok(())
}
