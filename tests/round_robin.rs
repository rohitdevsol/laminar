use std::sync::atomic::{AtomicBool, AtomicUsize};

use laminar::{
    config::types::BackendServerConfig,
    state::{app::UpstreamPool, backend::BackendState},
};

fn create_backend(id: &str, port: u16) -> BackendState {
    BackendState {
        config: BackendServerConfig {
            id: id.to_string(),
            host: "127.0.0.1".to_string(),
            port,
            weight: 1,
        },

        healthy: AtomicBool::new(true),
        active_connections: (0).into(),
        failed_health_checks: 0,
        failed_requests: AtomicUsize::new(0),
        total_requests: AtomicUsize::new(0),
        draining: AtomicBool::new(false),
    }
}

#[test]
fn round_robin_rotates_backends() {
    let upstream = UpstreamPool {
        id: "main".to_string(),
        current_index: (0).into(),
        algorithm: laminar::config::LoadBalancingAlgorithm::RoundRobin,
        backends: vec![
            create_backend("server-1", 9001).into(),
            create_backend("server-2", 9002).into(),
        ],
    };

    let first = upstream.next_backend().unwrap();

    let second = upstream.next_backend().unwrap();

    let third = upstream.next_backend().unwrap();

    assert_eq!(first.config.port, 9001);

    assert_eq!(second.config.port, 9002);

    assert_eq!(third.config.port, 9001);
}
