use laminar::{config::types::BackendServerConfig, state::backend::BackendState};
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};

#[test]
fn request_metrics_increment_correctly() {
    let backend = Arc::new(BackendState {
        config: BackendServerConfig {
            id: "server-1".into(),
            host: "127.0.0.1".into(),
            port: 9001,
            weight: 1,
        },
        healthy: AtomicBool::new(true),
        active_connections: AtomicUsize::new(0),
        total_requests: AtomicUsize::new(0),
        failed_requests: AtomicUsize::new(0),
        failed_health_checks: 0,
        draining: AtomicBool::new(false),
    });

    backend.increment_total_requests();
    backend.increment_total_requests();
    backend.increment_failed_requests();

    assert_eq!(backend.total_requests.load(Ordering::Relaxed), 2);
    assert_eq!(backend.failed_requests.load(Ordering::Relaxed), 1);
}
