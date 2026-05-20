use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};

use laminar::{
    config::types::BackendServerConfig,
    state::backend::{BackendState, ConnectionGuard},
};

#[test]
fn connection_guard_tracks_active_connections() {
    let backend = Arc::new(BackendState {
        config: BackendServerConfig {
            id: "server-1".to_string(),
            host: "127.0.0.1".to_string(),
            port: 9001,
            weight: 1,
        },
        healthy: AtomicBool::new(true),
        active_connections: AtomicUsize::new(0),
        failed_health_checks: 0,
    });

    assert_eq!(backend.active_connections.load(Ordering::Relaxed), 0);
    {
        let _guard = ConnectionGuard::new(backend.clone());
        assert_eq!(backend.active_connections.load(Ordering::Relaxed), 1);
    }

    assert_eq!(backend.active_connections.load(Ordering::Relaxed), 0);
}
