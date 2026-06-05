use std::sync::{Arc, atomic::AtomicUsize};

use anyhow::Result;

use crate::{
    config::{loader::load_config, validator::validate_config},
    state::{
        app::{SharedAppState, UpstreamPool},
        backend::BackendState,
    },
};

pub async fn reload_config(state: SharedAppState) -> Result<()> {
    let config_path = {
        let state = state.read().await;
        state.config_path.clone()
    };

    let config = load_config(&config_path)?;
    validate_config(&config)?;
    let mut state = state.write().await;

    for new_upstream in config.upstreams {
        let existing_upstream = state.upstreams.iter_mut().find(|u| u.id == new_upstream.id);
        match existing_upstream {
            Some(upstream) => {
                for server in &new_upstream.servers {
                    let exists = upstream.backends.iter().any(|b| b.config.id == server.id);

                    if !exists {
                        tracing::info!(
                            backend_id = %server.id,
                            "adding new backend"
                        );
                        upstream.backends.push(Arc::new(BackendState::new(server.clone())));
                    }
                }

                for backend in &upstream.backends {
                    let still_exists =
                        new_upstream.servers.iter().any(|s| s.id == backend.config.id);

                    if !still_exists {
                        backend.mark_draining();
                        tracing::info!(
                            backend_id =
                                %backend.config.id,
                            "backend marked draining during reload"
                        );
                    }
                }
                upstream.rebuild_weighted_backends();
            }

            None => {
                tracing::info!(
                    upstream_id = %new_upstream.id,
                    "adding new upstream"
                );

                let backends = new_upstream
                    .servers
                    .into_iter()
                    .map(|server| Arc::new(BackendState::new(server)))
                    .collect();

                let mut upstream_pool = UpstreamPool {
                    id: new_upstream.id,

                    current_index: AtomicUsize::new(0),

                    algorithm: new_upstream.algorithm,

                    backends,

                    weighted_backends: Vec::new(),
                };

                upstream_pool.rebuild_weighted_backends();

                state.upstreams.push(upstream_pool);
            }
        }
    }

    tracing::info!("runtime config reloaded");

    Ok(())
}
