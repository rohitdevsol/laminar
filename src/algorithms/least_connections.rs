use crate::state::backend::BackendState;
use std::sync::{Arc, atomic::Ordering};

pub fn select_backend(backends: &[Arc<BackendState>]) -> Option<Arc<BackendState>> {
    // prev and curr approach

    let mut selected: Option<Arc<BackendState>> = None;

    for backend in backends {
        //check if this backend is healthy
        if backend.healthy.load(Ordering::Relaxed) && !backend.draining.load(Ordering::Relaxed) {
            continue;
        }

        match &selected {
            Some(prev) => {
                let prev_connections = prev.active_connections.load(Ordering::Relaxed);

                let backend_connections = backend.active_connections.load(Ordering::Relaxed);

                if prev_connections > backend_connections {
                    selected = Some(backend.clone());
                }
            }
            None => {
                selected = Some(backend.clone());
            }
        }
    }

    selected
}
