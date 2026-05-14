use serde::Deserialize;

// Root configuration object for the entire load balancer.
// - listener config
// - load balancer - behavior
// - upstreams
#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub load_balancer: LoadBalancerConfig,
    // backends will be multiple and they will be arranged in upstream group
    pub upstreams: Vec<UpstreamConfig>, // and each upstream group can have multiple servers
}

// Configuration for the load balancer listener itself.
// This controls where Laminar accepts incoming client traffic.
#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

// Global behavior configuration shared across the load balancer runtime.
// These settings affect request routing and resiliency behavior.
#[derive(Debug, Deserialize)]
pub struct LoadBalancerConfig {
    pub retry_attempts: usize,
    pub sticky_sessions: bool,
}

// Static backend server definition loaded from configuration.
// This only contains immutable backend metadata.
// Live runtime information is tracked separately in "BackendState".
#[derive(Debug, Deserialize)]
pub struct BackendServerConfig {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub weight: usize,
}

// Represents a backend service pool.
// Each upstream contains:
// - a balancing algorithm
// - a collection of backend servers
// Requests are routed to one backend within the upstream.
#[derive(Debug, Deserialize)]
pub struct UpstreamConfig {
    pub id: String,
    pub algorithm: LoadBalancingAlgorithm,
    pub servers: Vec<BackendServerConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LoadBalancingAlgorithm {
    RoundRobin,
    LeastConnections,
    WeightedRoundRobin,
    IpHash,
    Random,
}
