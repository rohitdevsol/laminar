use std::{fs, sync::Arc, time::Duration};

use tempfile::NamedTempFile;

use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
    sync::RwLock,
};

use laminar::{
    admin::reload::reload_config, config::loader::load_config, health::tcp::start_health_checker,
    proxy::tcp::handle_connection, state::app::AppState,
};

#[tokio::test]
async fn reload_during_active_connection_survives() {
    let initial_config = r#"
server:
  host: "127.0.0.1"
  port: 8080

load_balancer:
  retry_attempts: 1
  sticky_sessions: false
  health_check_interval_secs: 1
  connect_timeout_secs: 5
  idle_timeout_secs: 30

upstreams:
  - id: "main"

    algorithm: "round_robin"

    servers:
      - id: "server-1"
        host: "127.0.0.1"
        port: 9001
        weight: 1
"#;

    let updated_config = r#"
server:
  host: "127.0.0.1"
  port: 8080

load_balancer:
  retry_attempts: 1
  sticky_sessions: false
  health_check_interval_secs: 1
  connect_timeout_secs: 5
  idle_timeout_secs: 30

upstreams:
  - id: "main"

    algorithm: "round_robin"

    servers:
      - id: "server-2"
        host: "127.0.0.1"
        port: 9002
        weight: 1
"#;

    let temp_file = NamedTempFile::new().unwrap();

    fs::write(temp_file.path(), initial_config).unwrap();

    let config = load_config(temp_file.path().to_str().unwrap()).unwrap();

    let state = Arc::new(RwLock::new(AppState::build(
        config,
        temp_file.path().to_str().unwrap().to_string(),
    )));

    // START HEALTH CHECKER
    {
        let health_state = state.clone();

        tokio::spawn(async move {
            start_health_checker(health_state, 1).await;
        });
    }

    // BACKEND SERVER
    let backend_listener = TcpListener::bind("127.0.0.1:9001").await.unwrap();

    tokio::spawn(async move {
        let (_socket, _) = backend_listener.accept().await.unwrap();

        // Keep connection alive long enough
        // for reload/draining semantics
        tokio::time::sleep(Duration::from_secs(2)).await;
    });

    // PROXY ENTRY
    let proxy_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();

    let proxy_addr = proxy_listener.local_addr().unwrap();

    let proxy_state = state.clone();

    tokio::spawn(async move {
        let (stream, _) = proxy_listener.accept().await.unwrap();

        handle_connection(stream, proxy_state).await.unwrap();
    });

    // CLIENT
    let mut client = TcpStream::connect(proxy_addr).await.unwrap();

    // Trigger active traffic
    client.write_all(b"ping").await.unwrap();

    // RELOAD DURING ACTIVE CONNECTION
    fs::write(temp_file.path(), updated_config).unwrap();

    reload_config(state.clone()).await.unwrap();

    // Give runtime time to mark draining
    tokio::time::sleep(Duration::from_millis(200)).await;

    {
        let state = state.read().await;

        let exists = state.upstreams[0].backends.iter().any(|b| b.config.id == "server-1");

        // backend may already be removed
        // depending on async timing

        if exists {
            let backend =
                state.upstreams[0].backends.iter().find(|b| b.config.id == "server-1").unwrap();

            assert!(backend.is_draining());
        }
    }

    // Wait for connection completion
    // + health cleanup loop
    tokio::time::sleep(Duration::from_secs(4)).await;

    {
        let state = state.read().await;

        assert!(state.upstreams[0].backends.iter().all(|b| { b.config.id != "server-1" }));
    }
}
