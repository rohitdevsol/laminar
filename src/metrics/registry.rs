use prometheus::{
    Encoder, HistogramOpts, HistogramVec, IntCounterVec, IntGaugeVec, Opts, Registry, TextEncoder,
};
use std::sync::OnceLock;
pub static REGISTRY: OnceLock<Registry> = OnceLock::new();
pub static TOTAL_REQUESTS: OnceLock<IntCounterVec> = OnceLock::new();
pub static FAILED_REQUESTS: OnceLock<IntCounterVec> = OnceLock::new();
pub static ACTIVE_CONNECTIONS: OnceLock<IntGaugeVec> = OnceLock::new();

pub static REQUEST_DURATION: OnceLock<HistogramVec> = OnceLock::new();
pub static BACKEND_CONNECT_DURATION: OnceLock<HistogramVec> = OnceLock::new();
pub static BYTES_IN: OnceLock<IntCounterVec> = OnceLock::new();
pub static BYTES_OUT: OnceLock<IntCounterVec> = OnceLock::new();

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

    let request_duration = HistogramVec::new(
        HistogramOpts::new("laminar_request_duration_seconds", "Request duration in seconds"),
        &["backend"],
    )
    .unwrap();

    let backend_connect_duration = HistogramVec::new(
        HistogramOpts::new("laminar_backend_connect_duration_seconds", "Backend connect duration"),
        &["backend"],
    )
    .unwrap();

    let bytes_in = IntCounterVec::new(
        Opts::new("laminar_bytes_in_total", "Total inbound bytes"),
        &["backend"],
    )
    .unwrap();

    let bytes_out = IntCounterVec::new(
        Opts::new("laminar_bytes_out_total", "Total outbound bytes"),
        &["backend"],
    )
    .unwrap();

    registry.register(Box::new(total_requests.clone())).unwrap();
    registry.register(Box::new(failed_requests.clone())).unwrap();
    registry.register(Box::new(active_connections.clone())).unwrap();
    registry.register(Box::new(request_duration.clone())).unwrap();
    registry.register(Box::new(backend_connect_duration.clone())).unwrap();
    registry.register(Box::new(bytes_in.clone())).unwrap();
    registry.register(Box::new(bytes_out.clone())).unwrap();

    REGISTRY.set(registry).unwrap();
    TOTAL_REQUESTS.set(total_requests).unwrap();
    FAILED_REQUESTS.set(failed_requests).unwrap();
    ACTIVE_CONNECTIONS.set(active_connections).unwrap();
    REQUEST_DURATION.set(request_duration).unwrap();
    BACKEND_CONNECT_DURATION.set(backend_connect_duration).unwrap();
    BYTES_IN.set(bytes_in).unwrap();
    BYTES_OUT.set(bytes_out).unwrap();
}

pub fn gather_metrics() -> String {
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.get().unwrap().gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
