use std::net::SocketAddr;
use futures::future::join_all;
use tokio::net::{ TcpListener, TcpStream };
use laminar::common::{ parse_yaml };

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

    let servers = parse_yaml().unwrap();

    let futures = servers.into_iter().map(|mut s| async move {
        let addr: SocketAddr = s.ip.parse().unwrap();
        s.can_connect = TcpStream::connect(addr).await.is_ok();
        return s;
    });

    let result = join_all(futures).await;

    assert!(result[0].can_connect);
    assert!(result[1].can_connect);
    assert!(!result[2].can_connect);
}
