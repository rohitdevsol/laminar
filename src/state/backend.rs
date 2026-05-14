// use std::sync::Arc;
// use tokio::sync::RwLock;
use crate::config::BackendServerConfig;

#[derive(Debug)]
pub struct BackendState {
    pub config: BackendServerConfig,
    pub healthy: bool,
    pub active_connections: usize,
    pub failed_health_checks: usize,
}

// pub type SharedBackendState = Arc<RwLock<BackendState>>;

impl BackendState {
    pub fn new(config: BackendServerConfig) -> Self {
        Self {
            config,
            healthy: true,
            active_connections: 0,
            failed_health_checks: 0,
        }
    }
}
