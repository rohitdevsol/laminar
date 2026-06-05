use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};

use laminar::{
    algorithms::round_robin, config::types::BackendServerConfig, state::backend::BackendState,
};

#[test]
fn unhealthy_backend_is_not_selected_again() {
    let backend_1 = Arc::new(BackendState {
        config: BackendServerConfig {
            id: "dead-server".to_string(),
            host: "127.0.0.1".to_string(),
            port: 9001,
            weight: 1,
        },

        healthy: AtomicBool::new(false),
        active_connections: AtomicUsize::new(0),
        failed_health_checks: 0,
        failed_requests: AtomicUsize::new(0),
        total_requests: AtomicUsize::new(0),
    });

    let backend_2 = Arc::new(BackendState {
        config: BackendServerConfig {
            id: "healthy-server".to_string(),
            host: "127.0.0.1".to_string(),
            port: 9002,
            weight: 1,
        },

        healthy: AtomicBool::new(true),
        active_connections: AtomicUsize::new(0),
        failed_health_checks: 0,
        failed_requests: AtomicUsize::new(0),
        total_requests: AtomicUsize::new(0),
    });

    let backends = vec![backend_1, backend_2.clone()];
    let index = AtomicUsize::new(0);
    let selected = round_robin::select_backend(&backends, &index).unwrap();
    assert_eq!(selected.config.id, "healthy-server");
    assert!(selected.healthy.load(Ordering::Relaxed));
}
