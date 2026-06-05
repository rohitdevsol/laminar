use crate::{metrics::registry::gather_metrics, state::app::SharedAppState};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use std::sync::atomic::Ordering;

use serde::Serialize;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct BackendMetrics {
    id: String,
    healthy: bool,
    active_connections: usize,
    total_requests: usize,
    failed_requests: usize,
    draining: bool,
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

async fn prometheus_handler() -> String {
    gather_metrics()
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
                    draining: backend.draining.load(Ordering::Relaxed),
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

async fn drain_backend_handler(
    Path(id): Path<String>,
    State(state): State<SharedAppState>,
) -> String {
    let state = state.read().await;

    for upstream in &state.upstreams {
        for backend in &upstream.backends {
            if backend.config.id == id {
                backend.mark_draining();

                tracing::info!(
                    backend_id = %id,
                    "backend marked as draining"
                );

                return format!("backend '{id}' marked draining");
            }
        }
    }

    format!("backend '{id}' not found")
}

pub async fn start_admin_server(address: &str, state: SharedAppState) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/metrics", get(metrics_handler))
        .route("/backend/{id}/drain", post(drain_backend_handler))
        .route("/prometheus", get(prometheus_handler))
        .with_state(state);
    let listener = TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
