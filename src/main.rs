use std::sync::{ Arc };

use anyhow::{ bail, Result };
use tokio::sync::RwLock;
use tracing::info;
use laminar::{
    config::{ loader::load_config, validator::validate_config },
    proxy::tcp::start_tcp_proxy,
    state::app::{ AppState, SharedAppState },
};

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
    let listener_address = format!("{}:{}", listener_host, listener_port);

    start_tcp_proxy(&listener_address, shared_state).await?;

    Ok(())
}
