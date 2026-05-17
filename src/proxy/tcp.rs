use crate::state::app::SharedAppState;
use crate::state::backend::ConnectionGuard;
use std::time::Duration;
use tokio::{
    io::copy_bidirectional,
    net::{TcpListener, TcpStream},
    time::timeout,
};
use tracing::{error, info};

pub async fn start_tcp_proxy(address: &str, state: SharedAppState) -> anyhow::Result<()> {
    let listener = TcpListener::bind(address).await?;

    info!("tcp proxy listening on {}", address);

    loop {
        let (client_stream, client_address) = listener.accept().await?;

        info!("new client connected {}", client_address);
        let state = state.clone();
        tokio::spawn(async move {
            if let Err(error) = handle_connection(client_stream, state).await {
                error!("connection handling failed {:?}", error)
            }
        });
    }
}

async fn handle_connection(mut stream: TcpStream, state: SharedAppState) -> anyhow::Result<()> {
    let (retry_attempt, connect_timeout, idle_timeout) = {
        let state = state.read().await;
        (state.retry_attempts, state.connect_timeout, state.idle_timeout)
    };

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
                    error!("no healthy backend available");
                    return Ok(());
                }
            };
            ConnectionGuard::new(backend_arc)
            // format!("{}:{}", backend.config.host, backend.config.port)
        };
        let backend_address = guard.address();

        info!("forwarding traffic to {}", backend_address);

        match proxy_connection(&mut stream, &backend_address, connect_timeout, idle_timeout).await {
            Ok(_) => {
                return Ok(());
            }
            Err(error) => {
                guard.mark_backend_unhealthy();
                error!("backend {} failed: {:?}", backend_address, error);
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
