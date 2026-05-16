use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize},
};

use laminar::{
    algorithms::least_connections, config::types::BackendServerConfig, state::backend::BackendState,
};

fn create_backend(
    id: &str,
    port: u16,
    healthy: bool,
    active_connections: usize,
) -> Arc<BackendState> {
    Arc::new(BackendState {
        config: BackendServerConfig {
            id: id.to_string(),
            host: "127.0.0.1".to_string(),
            port,
            weight: 1,
        },

        healthy: AtomicBool::new(healthy),
        active_connections: AtomicUsize::new(active_connections),
        failed_health_checks: 0,
    })
}

#[test]
fn selects_backend_with_fewer_connections() {
    let backend_1 = create_backend("server-1", 9001, true, 10);

    let backend_2 = create_backend("server-2", 9002, true, 2);

    let backends = vec![backend_1, backend_2];

    let selected = least_connections::select_backend(&backends).unwrap();

    assert_eq!(selected.config.id, "server-2");
}
