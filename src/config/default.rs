pub const DEFAULT_CONFIG: &str = r#"
version: 1

server:
  host: "0.0.0.0"
  port: 8080

load_balancer:
  retry_attempts: 2
  sticky_sessions: false

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
