use std::sync::{Arc, atomic::AtomicUsize};

use laminar::{
    algorithms::weighted_round_robin, config::types::BackendServerConfig,
    state::backend::BackendState,
};

#[test]
fn weighted_backend_selected_more_often() {
    let backend_1 = Arc::new(BackendState::new(BackendServerConfig {
        id: "server-1".into(),
        host: "127.0.0.1".into(),
        port: 9001,
        weight: 5,
    }));

    let backend_2 = Arc::new(BackendState::new(BackendServerConfig {
        id: "server-2".into(),
        host: "127.0.0.1".into(),
        port: 9002,
        weight: 1,
    }));

    let backends = vec![backend_1, backend_2];

    let counter = AtomicUsize::new(0);

    let mut server_1_hits = 0;
    let mut server_2_hits = 0;

    for _ in 0..60 {
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

    // println!("Server 1 hits: {}", server_1_hits);
    // println!("Server 2 hits: {}", server_2_hits);

    assert!(server_1_hits > server_2_hits);
}
