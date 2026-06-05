use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize},
};

use laminar::{
    algorithms::weighted_round_robin, config::types::BackendServerConfig,
    state::backend::BackendState,
};

fn create_backend(id: &str, weight: usize, healthy: bool, draining: bool) -> Arc<BackendState> {
    Arc::new(BackendState {
        config: BackendServerConfig { id: id.into(), host: "127.0.0.1".into(), port: 8080, weight },

        healthy: AtomicBool::new(healthy),
        draining: AtomicBool::new(draining),
        active_connections: AtomicUsize::new(0),
        total_requests: AtomicUsize::new(0),
        failed_requests: AtomicUsize::new(0),
        failed_health_checks: 0,
    })
}

#[test]
fn weighted_distribution_is_respected() {
    let backend_1 = create_backend("server-1", 5, true, false);
    let backend_2 = create_backend("server-2", 1, true, false);
    let backends = vec![backend_1, backend_2];
    let counter = AtomicUsize::new(0);

    let mut server_1_hits = 0;
    let mut server_2_hits = 0;

    for _ in 0..600 {
        let backend = weighted_round_robin::select_backend(&backends, &counter).unwrap();
        match backend.config.id.as_str() {
            "server-1" => {
                server_1_hits += 1;
            }
            "server-2" => {
                server_2_hits += 1;
            }
            _ => {}
        }
    }

    assert!(server_1_hits > 450);
    assert!(server_2_hits < 150);
}

#[test]
fn unhealthy_backend_is_skipped() {
    let backend_1 = create_backend("dead", 5, false, false);
    let backend_2 = create_backend("healthy", 1, true, false);
    let backends = vec![backend_1, backend_2];
    let counter = AtomicUsize::new(0);

    for _ in 0..20 {
        let backend = weighted_round_robin::select_backend(&backends, &counter).unwrap();
        assert_eq!(backend.config.id, "healthy");
    }
}

#[test]
fn draining_backend_is_skipped() {
    let backend_1 = create_backend("draining", 5, true, true);
    let backend_2 = create_backend("healthy", 1, true, false);
    let backends = vec![backend_1, backend_2];
    let counter = AtomicUsize::new(0);

    for _ in 0..20 {
        let backend = weighted_round_robin::select_backend(&backends, &counter).unwrap();
        assert_eq!(backend.config.id, "healthy");
    }
}

#[test]
fn returns_none_when_all_backends_invalid() {
    let backend_1 = create_backend("dead", 5, false, false);
    let backend_2 = create_backend("draining", 1, true, true);
    let backends = vec![backend_1, backend_2];
    let counter = AtomicUsize::new(0);
    let backend = weighted_round_robin::select_backend(&backends, &counter);

    assert!(backend.is_none());
}
