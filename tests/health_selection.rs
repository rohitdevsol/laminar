use std::sync::atomic::{AtomicBool, AtomicUsize};

use laminar::{
    config::types::BackendServerConfig,
    state::{app::UpstreamPool, backend::BackendState},
};

fn create_backend(id: &str, port: u16, healthy: bool) -> BackendState {
    BackendState {
        config: BackendServerConfig {
            id: id.to_string(),
            host: "127.0.0.1".to_string(),
            port,
            weight: 1,
        },

        healthy: AtomicBool::new(healthy),
        active_connections: (0).into(),
        failed_health_checks: 0,
        failed_requests: AtomicUsize::new(0),
        total_requests: AtomicUsize::new(0),
    }
}

#[test]
fn unhealthy_backend_is_skipped() {
    let upstream = UpstreamPool {
        id: "main".to_string(),
        current_index: (0).into(),
        algorithm: laminar::config::LoadBalancingAlgorithm::LeastConnections,
        backends: vec![
            create_backend("dead", 9001, false).into(),
            create_backend("healthy", 9002, true).into(),
        ],
    };

    let backend = upstream.next_backend().unwrap();

    assert_eq!(backend.config.port, 9002);
}

#[test]
fn returns_none_when_all_backends_dead() {
    let upstream = UpstreamPool {
        id: "main".to_string(),
        current_index: (0).into(),
        algorithm: laminar::config::LoadBalancingAlgorithm::LeastConnections,
        backends: vec![
            create_backend("dead-1", 9001, false).into(),
            create_backend("dead-2", 9002, false).into(),
        ],
    };

    let backend = upstream.next_backend();

    assert!(backend.is_none());
}
