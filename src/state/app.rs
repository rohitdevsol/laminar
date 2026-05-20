use crate::algorithms::{least_connections, round_robin};
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
}

pub type SharedAppState = Arc<RwLock<AppState>>;

impl AppState {
    pub fn build(config: Config) -> Self {
        let upstreams = config
            .upstreams
            .into_iter()
            .map(|upstream| {
                let backends =
                    upstream.servers.into_iter().map(|s| Arc::new(BackendState::new(s))).collect();

                UpstreamPool {
                    id: upstream.id,
                    current_index: AtomicUsize::new(0),
                    algorithm: upstream.algorithm,
                    backends,
                }
            })
            .collect();

        Self {
            upstreams,
            retry_attempts: config.load_balancer.retry_attempts,
            connect_timeout: Duration::from_secs(config.load_balancer.connect_timeout_secs),
            idle_timeout: Duration::from_secs(config.load_balancer.idle_timeout_secs),
        }
    }

    pub fn total_connections(&self) -> usize {
        self.upstreams
            .iter()
            .flat_map(|u| &u.backends)
            .map(|b| b.active_connections.load(std::sync::atomic::Ordering::Relaxed))
            .sum()
    }
}
