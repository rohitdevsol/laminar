use std::{sync::atomic::Ordering, time::Duration};

use crate::state::{app::SharedAppState, backend::BackendState};
use anyhow::Result;
use tokio::{net::TcpStream, time::sleep};
use tracing::info;

// This will evolve later into:
// - retries
// - thresholds
// - latency tracking
// - richer health states
pub async fn check_backend_status(backend: &BackendState) -> Result<()> {
    let backend_address = { format!("{}:{}", backend.config.host, backend.config.port) };

    match TcpStream::connect(&backend_address).await {
        Ok(_) => {
            backend.healthy.store(true, Ordering::Relaxed);
            info!("backend {} healthy", backend.config.id);
        }
        Err(_) => {
            backend.healthy.store(false, Ordering::Relaxed);
            info!("backend {} unreachable", backend.config.id);
        }
    }

    Ok(())
}

pub async fn start_health_checker(state: SharedAppState) {
    loop {
        let state = state.read().await;
        for upstream in &state.upstreams {
            for backend in &upstream.backends {
                let _ = check_backend_status(backend).await;
            }
        }
        drop(state);
        sleep(Duration::from_secs(5)).await;
    }
}
