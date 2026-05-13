use tokio::net::{ TcpListener };
use laminar::common::{ check_servers_health };

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

    let result = check_servers_health().await.unwrap();

    assert!(result[0].can_connect);
    assert!(result[1].can_connect);
    assert!(!result[2].can_connect);
}
