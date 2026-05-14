// // server state will have .. ip and port ( we can get that form the siurces.yml , check if it is active (up) and the number of connections )

// pub mod types;
// use std::{ fs::File, net::SocketAddr };
// use std::result::Result;
// use anyhow::{ self, Ok };
// use futures::future::join_all;
// use tokio::net::TcpStream;

// pub async fn check_servers_health() -> Result<Vec<Server>, anyhow::Error> {
//     let servers = parse_yaml()?;

//     // try to connect each one and filter out the dead ones
//     let futures = servers.into_iter().map(|mut s| async {
//         let addr: SocketAddr = s.ip.parse().expect("the address is not valid");
//         // let res = TcpStream::connect(addr).await;

//         // match res {
//         //     Ok(_) => {
//         //         s.can_connect = true;
//         //     }
//         //     Err(_) => {
//         //         s.can_connect = false;
//         //     }
//         // }

//         // shortcut
//         s.can_connect = TcpStream::connect(addr).await.is_ok();

//         s
//     });

//     // run the futures as the async blocks are lazy
//     let res = join_all(futures).await;
//     Ok(res)
// }

// pub async fn active_servers() -> Result<Vec<Server>, anyhow::Error> {
//     let servers = check_servers_health().await?;

//     Ok(
//         servers
//             .into_iter()
//             .filter(|item| item.can_connect)
//             .collect()
//     )
// }
