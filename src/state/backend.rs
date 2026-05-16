use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

// use std::sync::Arc;
// use tokio::sync::RwLock;
use crate::config::BackendServerConfig;

pub struct ConnectionGuard {
    // We hold an Arc so backend state stays alive
    // for the lifetime of the connection.
    backend: Arc<BackendState>,
}

// Mutable runtime representation of a backend server.
// - immutable backend configuration
// - mutable runtime health/connection state
// This struct is the primary object used during balancing decisions.
#[derive(Debug)]
pub struct BackendState {
    pub config: BackendServerConfig,

    // Temporary boolean health indicator.
    // This will later evolve into a richer health state machine:
    // Healthy -> Unhealthy -> Recovering
    pub healthy: AtomicBool,

    // This becomes important for least-connections balancing.
    pub active_connections: AtomicUsize,
    pub failed_health_checks: usize,
}

impl ConnectionGuard {
    pub fn new(backend: Arc<BackendState>) -> Self {
        // Increment immediately upon creation
        backend.active_connections.fetch_add(1, Ordering::Relaxed);
        Self { backend }
    }

    pub fn backend_id(&self) -> &str {
        &self.backend.config.id
    }
    // get the address from the guard
    pub fn address(&self) -> String {
        format!("{}:{}", self.backend.config.host, self.backend.config.port)
    }
    pub fn mark_backend_unhealthy(&self) {
        self.backend.healthy.store(false, Ordering::Relaxed);
    }
}

impl Drop for ConnectionGuard {
    fn drop(&mut self) {
        self.backend.active_connections.fetch_sub(1, Ordering::Relaxed);
    }
}

// pub type SharedBackendState = Arc<RwLock<BackendState>>;

impl BackendState {
    pub fn new(config: BackendServerConfig) -> Self {
        Self {
            config,
            healthy: AtomicBool::new(true),
            active_connections: AtomicUsize::new(0),
            failed_health_checks: 0,
        }
    }
}
