use crate::algorithms::{least_connections, round_robin, weighted_round_robin};
use crate::config::LoadBalancingAlgorithm;
use crate::{config::types::Config, state::backend::BackendState};
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::time::Duration;
use tokio::sync::RwLock;
// Contains all backend servers belonging to a single logical service.
#[derive(Debug)]
pub struct UpstreamPool {
    pub id: String,
    pub current_index: AtomicUsize,
    pub algorithm: LoadBalancingAlgorithm,
    pub backends: Vec<Arc<BackendState>>,
}

impl UpstreamPool {
    // Very naive round robin.
    pub fn next_backend(&self) -> Option<Arc<BackendState>> {
        match &self.algorithm {
            LoadBalancingAlgorithm::RoundRobin => {
                round_robin::select_backend(&self.backends, &self.current_index)
            }
            LoadBalancingAlgorithm::LeastConnections => {
                least_connections::select_backend(&self.backends)
            }
            LoadBalancingAlgorithm::WeightedRoundRobin => {
                weighted_round_robin::select_backend(&self.backends, &self.current_index)
            }
            _ => {
                unimplemented!("algorithm not implemented yet")
            }
        }
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
    pub retry_attempts: usize,
    pub upstreams: Vec<UpstreamPool>,
    pub connect_timeout: Duration,
    pub idle_timeout: Duration,
    pub config_path: String,
}

pub type SharedAppState = Arc<RwLock<AppState>>;

impl AppState {
    pub fn build(config: Config, config_path: String) -> Self {
        // config.upstreams is a grouped collection of upstreams
        // each upstream has an id, algorithm and servers( yes group of servers)
        // each server has id, host, port, weight

        let upstreams = config
            .upstreams
            .into_iter()
            .map(|upstream| {
                // upstream has id, algorithm and servers
                // BackendState.config is of same type as a server

                let backends =
                    upstream.servers.into_iter().map(|s| Arc::new(BackendState::new(s))).collect();

                UpstreamPool {
                    id: upstream.id,
                    current_index: AtomicUsize::new(0),
                    algorithm: upstream.algorithm,

                    backends, // all backends belonging to a single upstream type ( single logical service)
                }
            })
            .collect();

        Self {
            upstreams,
            retry_attempts: config.load_balancer.retry_attempts,
            connect_timeout: Duration::from_secs(config.load_balancer.connect_timeout_secs),
            idle_timeout: Duration::from_secs(config.load_balancer.idle_timeout_secs),
            config_path,
        }
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
