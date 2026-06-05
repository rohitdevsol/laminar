use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use tokio::net::TcpListener;

use laminar::{
    config::types::BackendServerConfig, health::tcp::check_backend_status,
    state::backend::BackendState,
};

fn create_backend(port: u16) -> BackendState {
    BackendState {
        config: BackendServerConfig {
            id: "test-backend".to_string(),
            host: "127.0.0.1".to_string(),
            port,
            weight: 1,
        },

        healthy: AtomicBool::new(false),
        active_connections: (0).into(),
        failed_health_checks: 0,
        failed_requests: AtomicUsize::new(0),
        total_requests: AtomicUsize::new(0),
        draining: AtomicBool::new(false),
    }
}

#[tokio::test]
async fn backend_becomes_healthy() {
    let listener = TcpListener::bind("127.0.0.1:9999").await.unwrap();

    tokio::spawn(async move {
        loop {
            let _ = listener.accept().await;
        }
    });

    let backend = create_backend(9999);

    check_backend_status(&backend).await.unwrap();

    assert!(backend.healthy.load(Ordering::Relaxed));
}

#[tokio::test]
async fn backend_becomes_unhealthy() {
    let backend = create_backend(9998);

    check_backend_status(&backend).await.unwrap();

    assert!(!backend.healthy.load(Ordering::Relaxed));
}
