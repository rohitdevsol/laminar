use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use crate::state::backend::BackendState;

pub fn select_backend(
    backends: &[Arc<BackendState>],
    current_index: &AtomicUsize,
) -> Option<Arc<BackendState>> {
    let mut weighted = Vec::new();

    for backend in backends {
        if !backend.is_routable() {
            continue;
        }

        for _ in 0..backend.config.weight {
            weighted.push(backend.clone());
        }
    }

    if weighted.is_empty() {
        return None;
    }

    let index = current_index.fetch_add(1, Ordering::Relaxed);

    Some(weighted[index % weighted.len()].clone())
}
