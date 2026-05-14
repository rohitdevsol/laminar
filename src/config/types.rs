use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub load_balancer: LoadBalancerConfig,
    // backends will be multiple and they will be arranged in upstream group
    pub upstreams: Vec<UpstreamConfig>, // and each upstream group can have multiple servers
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct LoadBalancerConfig {
    pub retry_attempts: usize,
    pub sticky_sessions: bool,
}

#[derive(Debug, Deserialize)]
pub struct BackendServerConfig {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub weight: usize,
}

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
