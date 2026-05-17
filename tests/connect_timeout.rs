use laminar::{
    config::types::BackendServerConfig,
    state::backend::{BackendState, ConnectionGuard},
};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::{net::TcpStream, time::timeout};

#[tokio::test]
async fn marks_backend_unhealthy_on_connect_timeout() {
    let backend = Arc::new(BackendState {
        config: BackendServerConfig {
            id: "dead-backend".into(),
            host: "10.255.255.1".into(),
            port: 1234,
            weight: 1,
        },

        healthy: AtomicBool::new(true),
        active_connections: AtomicUsize::new(0),
        failed_health_checks: 0,
    });

    let guard = ConnectionGuard::new(backend.clone());
    let address = guard.address();
    let result = timeout(Duration::from_millis(100), TcpStream::connect(address)).await;
    assert!(result.is_err());

    guard.mark_backend_unhealthy();
    assert!(!backend.healthy.load(Ordering::Relaxed));
}
