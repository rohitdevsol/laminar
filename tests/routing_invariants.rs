use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize},
};

use laminar::{
    algorithms::{least_connections, round_robin, weighted_round_robin},
    config::types::BackendServerConfig,
    state::backend::BackendState,
};

fn backend(id: &str, healthy: bool, draining: bool) -> Arc<BackendState> {
    Arc::new(BackendState {
        config: BackendServerConfig {
            id: id.into(),
            host: "127.0.0.1".into(),
            port: 8080,
            weight: 5,
        },

        healthy: AtomicBool::new(healthy),
        draining: AtomicBool::new(draining),
        active_connections: AtomicUsize::new(0),
        total_requests: AtomicUsize::new(0),
        failed_requests: AtomicUsize::new(0),
        failed_health_checks: 0,
    })
}

#[test]
fn draining_backend_never_routed() {
    let draining = backend("draining", true, true);
    let healthy = backend("healthy", true, false);
    let backends = vec![draining, healthy.clone()];
    let counter = AtomicUsize::new(0);

    for _ in 0..50 {
        let rr = round_robin::select_backend(&backends, &counter).unwrap();
        assert_eq!(rr.config.id, "healthy");
        let lc = least_connections::select_backend(&backends).unwrap();
        assert_eq!(lc.config.id, "healthy");
        let wrr = weighted_round_robin::select_backend(&backends, &counter).unwrap();
        assert_eq!(wrr.config.id, "healthy");
    }
}
