use std::{fs, sync::Arc};

use tempfile::NamedTempFile;

use tokio::sync::RwLock;

use laminar::{admin::reload::reload_config, config::loader::load_config, state::app::AppState};

#[tokio::test]
async fn reload_adds_new_backend() {
    let initial_config = r#"
server:
  host: "127.0.0.1"
  port: 8080

load_balancer:
  retry_attempts: 2
  sticky_sessions: false
  health_check_interval_secs: 5
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
  retry_attempts: 2
  sticky_sessions: false
  health_check_interval_secs: 5
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

    {
        let state = state.read().await;
        assert_eq!(state.upstreams[0].backends.len(), 1);
    }

    fs::write(temp_file.path(), updated_config).unwrap();
    reload_config(state.clone()).await.unwrap();

    {
        let state = state.read().await;
        assert_eq!(state.upstreams[0].backends.len(), 2);
        assert!(state.upstreams[0].backends.iter().any(|b| { b.config.id == "server-2" }));
    }
}
