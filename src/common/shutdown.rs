use tracing::info;

pub async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.expect("failed to listen for shutdown signal");
    info!("shutdown signal received");
}
