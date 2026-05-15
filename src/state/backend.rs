use std::sync::atomic::AtomicBool;

// use std::sync::Arc;
// use tokio::sync::RwLock;
use crate::config::BackendServerConfig;

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
    pub active_connections: usize,
    pub failed_health_checks: usize,
}

// pub type SharedBackendState = Arc<RwLock<BackendState>>;

impl BackendState {
    pub fn new(config: BackendServerConfig) -> Self {
        Self {
            config,
            healthy: AtomicBool::new(true),
            active_connections: 0,
            failed_health_checks: 0,
        }
    }
}
