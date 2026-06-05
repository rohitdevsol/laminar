use prometheus::{Encoder, IntCounterVec, IntGaugeVec, Registry, TextEncoder};
use std::sync::OnceLock;
pub static REGISTRY: OnceLock<Registry> = OnceLock::new();
pub static TOTAL_REQUESTS: OnceLock<IntCounterVec> = OnceLock::new();
pub static FAILED_REQUESTS: OnceLock<IntCounterVec> = OnceLock::new();
pub static ACTIVE_CONNECTIONS: OnceLock<IntGaugeVec> = OnceLock::new();

pub fn initialize_metrics() {
    let registry = Registry::new();

    let total_requests = IntCounterVec::new(
        prometheus::Opts::new("laminar_total_requests", "Total successful requests"),
        &["backend"],
    )
    .unwrap();

    let failed_requests = IntCounterVec::new(
        prometheus::Opts::new("laminar_failed_requests", "Total failed requests"),
        &["backend"],
    )
    .unwrap();

    let active_connections = IntGaugeVec::new(
        prometheus::Opts::new("laminar_active_connections", "Current active connections"),
        &["backend"],
    )
    .unwrap();

    registry.register(Box::new(total_requests.clone())).unwrap();
    registry.register(Box::new(failed_requests.clone())).unwrap();
    registry.register(Box::new(active_connections.clone())).unwrap();

    REGISTRY.set(registry).unwrap();
    TOTAL_REQUESTS.set(total_requests).unwrap();
    FAILED_REQUESTS.set(failed_requests).unwrap();
    ACTIVE_CONNECTIONS.set(active_connections).unwrap();
}

pub fn gather_metrics() -> String {
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.get().unwrap().gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
