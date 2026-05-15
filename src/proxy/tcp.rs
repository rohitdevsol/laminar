use anyhow::Ok;
use tokio::{
    io::copy_bidirectional,
    net::{TcpListener, TcpStream},
};
use tracing::{error, info};

use crate::state::app::SharedAppState;
use crate::state::backend::ConnectionGuard;

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
    let guard = {
        let state = state.read().await;
        let upstream = &state.upstreams[0];
        let backend_arc = match upstream.next_backend() {
            Some(backend) => backend.clone(),
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

    let mut backend_stream = TcpStream::connect(&backend_address).await?;

    copy_bidirectional(&mut stream, &mut backend_stream).await?;

    Ok(())
}
