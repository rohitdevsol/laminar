use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use crate::state::backend::BackendState;

pub fn select_backend(
    weighted_backends: &[Arc<BackendState>],
    current_index: &AtomicUsize,
) -> Option<Arc<BackendState>> {
    let routable = weighted_backends
        .iter()
        .filter(|backend| backend.is_routable())
        .cloned()
        .collect::<Vec<_>>();

    if routable.is_empty() {
        return None;
    }

    let index = current_index.fetch_add(1, Ordering::Relaxed);

    Some(routable[index % routable.len()].clone())
}
