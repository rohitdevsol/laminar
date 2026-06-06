use crate::{
    admin::reload::reload_config, metrics::registry::gather_metrics, state::app::SharedAppState,
};
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

#[derive(Serialize)]
struct BackendStatus {
    id: String,
    healthy: bool,
    draining: bool,
    weight: usize,
    active_connections: usize,
    total_requests: usize,
    failed_requests: usize,
}

#[derive(Serialize)]
struct UpstreamStatus {
    id: String,
    algorithm: String,
    backend_count: usize,
    weighted_backend_count: usize,
    backends: Vec<BackendStatus>,
}

#[derive(Serialize)]
struct StatusResponse {
    upstreams: Vec<UpstreamStatus>,
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

async fn reload_handler(State(state): State<SharedAppState>) -> String {
    match reload_config(state).await {
        Ok(_) => "config reloaded".into(),

        Err(error) => {
            tracing::error!(
                error = %error,
                "config reload failed"
            );

            format!("reload failed: {error}")
        }
    }
}

async fn status_handler(State(state): State<SharedAppState>) -> Json<StatusResponse> {
    let state = state.read().await;

    let upstreams = state
        .upstreams
        .iter()
        .map(|upstream| {
            let backends = upstream
                .backends
                .iter()
                .map(|backend| BackendStatus {
                    id: backend.config.id.clone(),
                    healthy: backend.healthy.load(Ordering::Relaxed),
                    draining: backend.draining.load(Ordering::Relaxed),
                    weight: backend.config.weight,
                    active_connections: backend.active_connections.load(Ordering::Relaxed),
                    total_requests: backend.total_requests.load(Ordering::Relaxed),
                    failed_requests: backend.failed_requests.load(Ordering::Relaxed),
                })
                .collect();

            UpstreamStatus {
                id: upstream.id.clone(),
                algorithm: format!("{:?}", upstream.algorithm),
                backend_count: upstream.backends.len(),
                weighted_backend_count: upstream.weighted_backends.len(),
                backends,
            }
        })
        .collect();

    Json(StatusResponse { upstreams })
}

async fn disable_backend_handler(
    Path(id): Path<String>,
    State(state): State<SharedAppState>,
) -> String {
    let state = state.read().await;

    for upstream in &state.upstreams {
        for backend in &upstream.backends {
            if backend.config.id == id {
                backend.disable();

                tracing::info!(
                    backend_id = %id,
                    "backend disabled"
                );

                return format!("backend '{id}' disabled");
            }
        }
    }

    format!("backend '{id}' not found")
}

async fn enable_backend_handler(
    Path(id): Path<String>,
    State(state): State<SharedAppState>,
) -> String {
    let state = state.read().await;

    for upstream in &state.upstreams {
        for backend in &upstream.backends {
            if backend.config.id == id {
                backend.enable();

                tracing::info!(
                    backend_id = %id,
                    "backend enabled"
                );

                return format!("backend '{id}' enabled");
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
        .route("/reload", post(reload_handler))
        .route("/status", get(status_handler))
        .route("/backend/{id}/disable", post(disable_backend_handler))
        .route("/backend/{id}/enable", post(enable_backend_handler))
        .with_state(state);
    let listener = TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
