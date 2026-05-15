use anyhow::Ok;
use tokio::{ io::copy_bidirectional, net::{ TcpListener, TcpStream } };
use tracing::{ error, info };

use crate::state::app::SharedAppState;

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
    let backend_address = {
        let state = state.read().await;
        let upstream = &state.upstreams[0];
        let backend = upstream.next_backend();
        format!("{}:{}", backend.config.host, backend.config.port)
    };

    info!("forwarding traffic to {}", backend_address);

    let mut backend_stream = TcpStream::connect(&backend_address).await?;

    drop(state);

    copy_bidirectional(&mut stream, &mut backend_stream).await?;

    Ok(())
}
