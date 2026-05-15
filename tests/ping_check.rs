use laminar::{config::types::BackendServerConfig, state::backend::BackendState};
use tokio::net::TcpListener;

#[tokio::test]
async fn ping_check() {
    let ports = vec![3000, 3001];

    for port in &ports {
        let listener = TcpListener::bind(("127.0.0.1", *port)).await.unwrap();

        tokio::spawn(async move {
            loop {
                let _ = listener.accept().await;
            }
        });
    }

    let backends = vec![
        BackendState::new(BackendServerConfig {
            id: "server-1".into(),
            host: "127.0.0.1".into(),
            port: 3000,
            weight: 1,
        }),
        BackendState::new(BackendServerConfig {
            id: "server-2".into(),
            host: "127.0.0.1".into(),
            port: 3001,
            weight: 1,
        }),
        BackendState::new(BackendServerConfig {
            id: "dead-server".into(),
            host: "127.0.0.1".into(),
            port: 3999,
            weight: 1,
        }),
    ];

    for backend in backends {
        let addr = format!("{}:{}", backend.config.host, backend.config.port);

        let result = tokio::net::TcpStream::connect(addr).await;

        if backend.config.port == 3999 {
            assert!(result.is_err());
        } else {
            assert!(result.is_ok());
        }
    }
}
