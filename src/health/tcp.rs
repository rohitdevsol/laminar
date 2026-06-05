use std::{sync::atomic::Ordering, time::Duration};

use crate::state::{app::SharedAppState, backend::BackendState};
use anyhow::Result;
use tokio::{net::TcpStream, time::sleep};
use tracing::{info, warn};

// This will evolve later into:
// - retries
// - thresholds
// - latency tracking
// - richer health states
pub async fn check_backend_status(backend: &BackendState) -> Result<()> {
    let backend_address = { format!("{}:{}", backend.config.host, backend.config.port) };

    let was_healthy = backend.healthy.load(Ordering::Relaxed);
    let is_healthy = TcpStream::connect(&backend_address).await.is_ok();

    backend.healthy.store(is_healthy, Ordering::Relaxed);

    if was_healthy != is_healthy {
        if is_healthy {
            info!("backend '{}' recovered", backend.config.id);
        } else {
            warn!("backend '{}' became unhealthy", backend.config.id);
        }
    }

    Ok(())
}

pub async fn start_health_checker(state: SharedAppState, interval_secs: u64) {
    loop {
        let backends = {
            let state = state.read().await;

            state
                .upstreams
                .iter()
                .flat_map(|upstream| upstream.backends.clone())
                .collect::<Vec<_>>()
        };
        let mut removable_backend_ids = Vec::new();

        for backend in backends {
            let _ = check_backend_status(&backend).await;
            if backend.is_draining() && backend.active_connections.load(Ordering::Relaxed) == 0 {
                info!(backend_id =%backend.config.id,"backend safe to remove");
                removable_backend_ids.push(backend.config.id.clone());
            }
        }

        if !removable_backend_ids.is_empty() {
            let mut state = state.write().await;

            for upstream in &mut state.upstreams {
                upstream.backends.retain(|backend| {
                    let should_remove = removable_backend_ids.contains(&backend.config.id);
                    if should_remove {
                        info!(backend_id =%backend.config.id,"backend removed from runtime");
                    }
                    !should_remove
                });
            }
        }

        sleep(Duration::from_secs(interval_secs)).await;
    }
}
