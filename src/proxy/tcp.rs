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
        let (client_stream, client_address) = listener.accept().await?;

        info!(
            client = %client_address,
            "new client connected"
        );
        let state = state.clone();
        tokio::spawn(async move {
            if let Err(error) = handle_connection(client_stream, state).await {
                error!("connection handling failed {:?}", error)
            }
        });
    }
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
        let guard = {
            let state = state.read().await;
            let upstream = &state.upstreams[0];
            let backend_arc = match upstream.next_backend() {
                Some(backend) => backend,
                None => {
                    error!(
                        request_id = %request_id,
                        "no healthy backend available"
                    );
                    return Ok(());
                }
            };
            ConnectionGuard::new(backend_arc)
            // format!("{}:{}", backend.config.host, backend.config.port)
        };
        let backend_address = guard.address();
        if attempted_backends.contains(guard.backend_id()) {
            continue;
        }

        info!(
            request_id = %request_id,
            backend_id = %guard.backend_id(),
            backend = %backend_address,
            "forwarding traffic"
        );

        match proxy_connection(&mut stream, &backend_address, connect_timeout, idle_timeout).await {
            Ok(_) => {
                info!(
                    request_id = %request_id,
                    backend_id = %guard.backend_id(),
                    "request completed"
                );
                guard.backend().increment_total_requests();
                return Ok(());
            }
            Err(error) => {
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
    error!("all backend retry attempts failed");
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
