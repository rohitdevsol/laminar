use std::sync::Arc;
use tokio::sync::RwLock;
use crate::{ config::types::Config, state::backend::BackendState };

#[derive(Debug)]
pub struct UpstreamPool {
    pub id: String,
    pub backends: Vec<BackendState>,
}

#[derive(Debug)]
pub struct AppState {
    pub upstreams: Vec<UpstreamPool>,
}

pub type SharedAppState = Arc<RwLock<AppState>>;

impl AppState {
    pub fn build(config: Config) -> Self {
        let upstreams = config.upstreams
            .into_iter()
            .map(|upstream| {
                let backends = upstream.servers.into_iter().map(BackendState::new).collect();

                UpstreamPool {
                    id: upstream.id,
                    backends,
                }
            })
            .collect();

        Self { upstreams }
    }
}
