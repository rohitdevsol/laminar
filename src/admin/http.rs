use std::sync::atomic::Ordering;

use axum::{Json, Router, extract::State, routing::get};

use laminar::state::app::SharedAppState;
use serde::Serialize;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct BackendMetrics {
    id: String,
    healthy: bool,
    active_connections: usize,
    total_requests: usize,
    failed_requests: usize,
}

#[derive(Serialize)]
struct UpstreamMetrics {
    id: String,
    algorithm: String,
    backends: Vec<BackendMetrics>,
}

#[derive(Serialize)]
struct MetricsResponse {
    upstreams: Vec<UpstreamMetrics>,
}

async fn metrics_handler(State(state): State<SharedAppState>) -> Json<MetricsResponse> {
    let state = state.read().await;

    let upstreams = state
        .upstreams
        .iter()
        .map(|upstream| {
            let backends = upstream
                .backends
                .iter()
                .map(|backend| BackendMetrics {
                    id: backend.config.id.clone(),
                    healthy: backend.healthy.load(Ordering::Relaxed),
                    active_connections: backend.active_connections.load(Ordering::Relaxed),
                    total_requests: backend.total_requests.load(Ordering::Relaxed),
                    failed_requests: backend.failed_requests.load(Ordering::Relaxed),
                })
                .collect();

            UpstreamMetrics {
                id: upstream.id.clone(),
                algorithm: format!("{:?}", upstream.algorithm),
                backends,
            }
        })
        .collect();

    Json(MetricsResponse { upstreams })
}

pub async fn start_admin_server(address: &str, state: SharedAppState) -> anyhow::Result<()> {
    let app = Router::new().route("/metrics", get(metrics_handler)).with_state(state);
    let listener = TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
