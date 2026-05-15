#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use anyhow::{ Result, bail };
use laminar::{
    config::{ loader::load_config, validator::validate_config },
    health::tcp::start_health_checker,
    proxy::tcp::start_tcp_proxy,
    state::app::{ AppState, SharedAppState },
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let path = std::env
        ::args()
        .nth(1)
        .unwrap_or_else(|| "laminar_config.yaml".to_string());

    info!("loading config from {}", path);
    let config = load_config(&path)?;

    validate_config(&config)?;
    info!("config validation successful");

    let listener_host = config.server.host.clone();

    let listener_port = config.server.port;

    let state = AppState::build(config);

    info!("initialized {} upstream pools", state.upstreams.len());

    if state.upstreams.is_empty() {
        bail!("no upstreams configured");
    }

    let shared_state: SharedAppState = Arc::new(RwLock::new(state));
    // for upstream in &state.upstreams {
    //     info!("upstream '{}' initialized with {} backends", upstream.id, upstream.backends.len());
    // }

    let health_state = shared_state.clone();

    tokio::spawn(async move {
        start_health_checker(health_state).await;
    });

    let listener_address = format!("{listener_host}:{listener_port}");

    start_tcp_proxy(&listener_address, shared_state).await?;

    Ok(())
}
