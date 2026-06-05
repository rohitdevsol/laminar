use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

// use std::sync::Arc;
// use tokio::sync::RwLock;
use crate::config::BackendServerConfig;
use crate::metrics::registry::ACTIVE_CONNECTIONS;
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

    pub total_requests: AtomicUsize,
    pub failed_requests: AtomicUsize,

    pub draining: AtomicBool,
}

impl ConnectionGuard {
    pub fn new(backend: Arc<BackendState>) -> Self {
        // Increment immediately upon creation
        backend.active_connections.fetch_add(1, Ordering::Relaxed);
        if let Some(metrics) = ACTIVE_CONNECTIONS.get() {
            metrics.with_label_values(&[&backend.config.id]).inc();
        }
        Self { backend }
    }

    pub fn backend_id(&self) -> &str {
        &self.backend.config.id
    }
    // get the address from the guard
    pub fn address(&self) -> String {
        format!("{}:{}", self.backend.config.host, self.backend.config.port)
    }

    pub fn backend(&self) -> &BackendState {
        &self.backend
    }
    pub fn mark_backend_unhealthy(&self) {
        self.backend.healthy.store(false, Ordering::Relaxed);
    }
}

impl Drop for ConnectionGuard {
    fn drop(&mut self) {
        self.backend.active_connections.fetch_sub(1, Ordering::Relaxed);
        if let Some(metrics) = ACTIVE_CONNECTIONS.get() {
            metrics.with_label_values(&[&self.backend.config.id]).dec();
        }
    }
}

// pub type SharedBackendState = Arc<RwLock<BackendState>>;

impl BackendState {
    pub fn new(config: BackendServerConfig) -> Self {
        Self {
            config,
            healthy: AtomicBool::new(true),
            draining: AtomicBool::new(false),
            active_connections: AtomicUsize::new(0),
            failed_health_checks: 0,
            total_requests: AtomicUsize::new(0),
            failed_requests: AtomicUsize::new(0),
        }
    }

    pub fn increment_total_requests(&self) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_failed_requests(&self) {
        self.failed_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn mark_draining(&self) {
        self.draining.store(true, Ordering::Relaxed);
    }

    pub fn is_draining(&self) -> bool {
        self.draining.load(Ordering::Relaxed)
    }

    pub fn is_healthy(&self) -> bool {
        self.healthy.load(Ordering::Relaxed)
    }

    pub fn is_routable(&self) -> bool {
        self.healthy.load(Ordering::Relaxed) && !self.draining.load(Ordering::Relaxed)
    }
}
