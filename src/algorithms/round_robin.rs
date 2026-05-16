use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use crate::state::backend::BackendState;

pub fn select_backend(
    backends: &[Arc<BackendState>],
    current_index: &AtomicUsize,
) -> Option<Arc<BackendState>> {
    for _ in 0..backends.len() {
        let index = current_index.fetch_add(1, Ordering::Relaxed);
        let backend = &backends[index % backends.len()];
        if backend.healthy.load(Ordering::Relaxed) {
            return Some(backend.clone());
        }
    }
    None
}
