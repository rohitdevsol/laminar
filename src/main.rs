use anyhow::{ bail, Result };
use tracing::info;
use laminar::{ config::{ loader::load_config, validator::validate_config }, state::app::AppState };

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

    let state = AppState::build(config);

    println!("{:#?}", state);
    info!("initialized {} upstream pools", state.upstreams.len());
    for upstream in &state.upstreams {
        info!("upstream '{}' initialized with {} backends", upstream.id, upstream.backends.len());
    }
    if state.upstreams.is_empty() {
        bail!("no upstreams configured");
    }

    Ok(())
}
