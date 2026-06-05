use crate::common::shutdown::shutdown_signal;
use crate::metrics::registry::{FAILED_REQUESTS, TOTAL_REQUESTS};
use crate::state::app::SharedAppState;
use crate::state::backend::ConnectionGuard;
use std::{collections::HashSet, time::Duration};
use tokio::{
    io::copy_bidirectional,
    net::{TcpListener, TcpStream},
    time::timeout,
};
use tracing::{error, info};
use uuid::Uuid;

pub async fn start_tcp_proxy(address: &str, state: SharedAppState) -> anyhow::Result<()> {
    let listener = TcpListener::bind(address).await?;

    info!("tcp proxy listening on {}", address);

    loop {
        tokio::select! {
            result = listener.accept() => {
                let (client_stream, client_address) = result?;
                info!(
                    client = %client_address,
                    "new client connected"
                );
                let state = state.clone();
                tokio::spawn(async move {
                    if let Err(error) = handle_connection(client_stream,state).await
                    {
                        error!(
                            error = %error,
                            "connection handling failed"
                        );
                    }
                });
            }

            _ = shutdown_signal() => {
                info!("tcp proxy shutting down");
                break;
            }
        }
    }
    info!("tcp proxy stopped accepting new connections");

    Ok(())
}

pub async fn handle_connection(mut stream: TcpStream, state: SharedAppState) -> anyhow::Result<()> {
    let request_id = Uuid::new_v4();
    let (retry_attempt, connect_timeout, idle_timeout) = {
        let state = state.read().await;
        (state.retry_attempts, state.connect_timeout, state.idle_timeout)
    };

    let mut attempted_backends = HashSet::new();

    // retry attempts mean how many times we should try
    // connecting the client to a suitable backend server
    //
    // if a backend is available:
    // route the traffic normally
    //
    // if connection fails:
    // mark that backend unhealthy so future selections skip it
    for _ in 0..retry_attempt {
        let (backend_arc, algorithm) = {
            let state = state.read().await;
            let upstream = &state.upstreams[0];
            let algorithm = upstream.algorithm.clone();
            match upstream.next_backend() {
                Some(backend) => (backend, algorithm),
                None => {
                    error!(
                        request_id = %request_id,
                        "no healthy backend available"
                    );

                    return Ok(());
                }
            }
        };
        if attempted_backends.contains(&backend_arc.config.id) {
            continue;
        }

        let guard = ConnectionGuard::new(backend_arc);
        let backend_address = guard.address();

        info!(
            request_id = %request_id,
            backend_id = %guard.backend_id(),
            backend = %backend_address,
            algorithm = ?algorithm,
            "proxy connection started"
        );

        match proxy_connection(&mut stream, &backend_address, connect_timeout, idle_timeout).await {
            Ok(_) => {
                info!(
                    request_id = %request_id,
                    backend_id = %guard.backend_id(),
                    "request completed"
                );
                if let Some(metrics) = TOTAL_REQUESTS.get() {
                    metrics.with_label_values(&[guard.backend_id()]).inc();
                }
                guard.backend().increment_total_requests();
                return Ok(());
            }
            Err(error) => {
                if let Some(metrics) = FAILED_REQUESTS.get() {
                    metrics.with_label_values(&[guard.backend_id()]).inc();
                }
                guard.backend().increment_failed_requests();
                guard.mark_backend_unhealthy();
                error!(
                    request_id = %request_id,
                    backend_id = %guard.backend_id(),
                    backend = %backend_address,
                    attempt = attempted_backends.len() + 1,
                    error = %error,
                    "backend request failed"
                );
                attempted_backends.insert(guard.backend_id().to_string());
                continue;
            }
        }
    }
    error!(
        request_id = %request_id,
        "all backend retry attempts failed"
    );
    Ok(())
}

async fn proxy_connection(
    client_stream: &mut TcpStream,
    backend_address: &str,
    connect_timeout: Duration,
    idle_timeout: Duration,
) -> anyhow::Result<()> {
    let mut backend_stream =
        timeout(connect_timeout, TcpStream::connect(backend_address)).await??;

    match timeout(idle_timeout, copy_bidirectional(client_stream, &mut backend_stream)).await {
        Ok(Ok(_)) => Ok(()),

        Ok(Err(error)) => {
            anyhow::bail!("proxy IO error: {error}");
        }

        Err(_) => {
            anyhow::bail!("connection idle timeout");
        }
    }
}
