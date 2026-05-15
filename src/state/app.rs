use crate::{config::types::Config, state::backend::BackendState};
use std::sync::Arc;
use tokio::sync::RwLock;

// Contains all backend servers belonging to a single logical service.
#[derive(Debug)]
pub struct UpstreamPool {
    pub id: String,
    pub current_index: usize,
    pub backends: Vec<BackendState>,
}

impl UpstreamPool {
    // Very naive round robin.
    pub fn next_backend(&mut self) -> &BackendState {
        let backend = &self.backends[self.current_index % self.backends.len()];
        self.current_index += 1;
        backend
    }
}
// Central shared runtime state for the entire load balancer.
// Most subsystems eventually interact with this:
// - proxying
// - health checks
// - balancing
// - metrics
// - admin APIs
#[derive(Debug)]
pub struct AppState {
    pub upstreams: Vec<UpstreamPool>,
}

pub type SharedAppState = Arc<RwLock<AppState>>;

impl AppState {
    pub fn build(config: Config) -> Self {
        // config.upstreams is a grouped collection of upstreams
        // each upstream has an id, algorithm and servers( yes group of servers)
        // each server has id, host, port, weight

        let upstreams = config
            .upstreams
            .into_iter()
            .map(|upstream| {
                // upstream has id, algorithm and servers
                // BackendState.config is of same type as a server

                let backends = upstream.servers.into_iter().map(BackendState::new).collect();

                UpstreamPool {
                    id: upstream.id,
                    current_index: 0,
                    backends, // all backends belonging to a single upstream type ( single logical service)
                }
            })
            .collect();

        Self { upstreams }
    }
}

// How will the AppState look in the end ( just a sample )
/*
  AppState {
    upstreams: [
        UpstreamPool {
            id: "main",
            backends: [
                BackendState {
                    config: BackendServerConfig {
                        id: "server-1",
                        host: "127.0.0.1",
                        port: 9001,
                        weight: 1,
                    },
                    healthy: true,
                    active_connections: 0,
                    failed_health_checks: 0,
                },
                BackendState {
                    config: BackendServerConfig {
                        id: "server-2",
                        host: "127.0.0.1",
                        port: 9002,
                        weight: 1,
                    },
                    healthy: true,
                    active_connections: 0,
                    failed_health_checks: 0,
                },
            ],
        },
    ],
}
 */
