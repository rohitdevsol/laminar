use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use tokio::time::timeout;

use laminar::config::types::{
    BackendServerConfig, Config, LoadBalancerConfig, LoadBalancingAlgorithm, ServerConfig,
    UpstreamConfig,
};
use laminar::proxy::tcp::handle_connection;
use laminar::state::app::AppState;

#[tokio::test]
async fn test_connect_timeout() {
    // We use a non-routable IP to simulate a connection timeout (black hole)
    let config = Config {
        server: ServerConfig { host: "127.0.0.1".into(), port: 8080 },
        load_balancer: LoadBalancerConfig {
            retry_attempts: 1,
            sticky_sessions: false,
            health_check_interval_secs: 10,
            connect_timeout_secs: 1, // 1 second timeout
            idle_timeout_secs: 5,
        },
        upstreams: vec![UpstreamConfig {
            id: "test".into(),
            algorithm: LoadBalancingAlgorithm::RoundRobin,
            servers: vec![BackendServerConfig {
                id: "bad-server".into(),
                host: "10.255.255.1".into(), // Non-routable
                port: 80,
                weight: 1,
            }],
        }],
    };

    let state = Arc::new(RwLock::new(AppState::build(config)));

    // Create a local listener to act as the "client" entry point
    let proxy_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let proxy_addr = proxy_listener.local_addr().unwrap();

    let state_clone = state.clone();
    tokio::spawn(async move {
        if let Ok((client_stream, _)) = proxy_listener.accept().await {
            let _ = handle_connection(client_stream, state_clone).await;
        }
    });

    // Connect as a client
    let _client = TcpStream::connect(proxy_addr).await.unwrap();

    // The handle_connection task should attempt to connect to 10.255.255.1,
    // timeout after 1s, and then exit (since retry_attempts=1).
    // We check that the backend is marked unhealthy.
    tokio::time::sleep(Duration::from_millis(1500)).await;

    let state_read = state.read().await;
    let backend = &state_read.upstreams[0].backends[0];
    assert!(!backend.is_healthy(), "Backend should be marked unhealthy after timeout");
}

#[tokio::test]
async fn test_idle_timeout() {
    // 1. Start a dummy backend that accepts but never sends/receives data
    let backend_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let backend_addr = backend_listener.local_addr().unwrap();

    let config = Config {
        server: ServerConfig { host: "127.0.0.1".into(), port: 8081 },
        load_balancer: LoadBalancerConfig {
            retry_attempts: 1,
            sticky_sessions: false,
            health_check_interval_secs: 10,
            connect_timeout_secs: 5,
            idle_timeout_secs: 1, // 1 second idle timeout
        },
        upstreams: vec![UpstreamConfig {
            id: "test".into(),
            algorithm: LoadBalancingAlgorithm::RoundRobin,
            servers: vec![BackendServerConfig {
                id: "idle-server".into(),
                host: backend_addr.ip().to_string(),
                port: backend_addr.port(),
                weight: 1,
            }],
        }],
    };

    let state = Arc::new(RwLock::new(AppState::build(config)));

    // 2. Start a local listener to act as the "client" entry point
    let proxy_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let proxy_addr = proxy_listener.local_addr().unwrap();

    let state_clone = state.clone();
    tokio::spawn(async move {
        let (client_stream, _) = proxy_listener.accept().await.unwrap();
        let _ = handle_connection(client_stream, state_clone).await;
    });

    // 3. Connect as a client
    let mut client = TcpStream::connect(proxy_addr).await.unwrap();

    // 4. The backend must accept the connection for copy_bidirectional to start
    let _backend_conn = backend_listener.accept().await.unwrap();

    // 5. Wait for timeout. The proxy should close the connection after 1s.
    // We try to read. If it's closed, read returns 0 or error.
    let mut buf = [0u8; 1];
    let result = timeout(Duration::from_secs(3), client.read(&mut buf)).await;

    assert!(result.is_ok(), "Read should not timeout, the proxy should close it");
    assert_eq!(
        result.unwrap().unwrap(),
        0,
        "Proxy should have closed the connection due to idle timeout"
    );
}
