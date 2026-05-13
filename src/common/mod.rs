// server state will have .. ip and port ( we can get that form the siurces.yml , check if it is active (up) and the number of connections )

use std::{ fs::File, net::SocketAddr };
use std::result::Result;
use anyhow;
use futures::future::join_all;
use tokio::net::TcpStream;

#[derive(Clone, Debug, Default)]
pub struct Server {
    pub ip: String,
    pub can_connect: bool,
    // maintain the numbe rof connections on each server
    // will help us in choosing the suitable server to handle the requests
    pub connections: usize,
}

pub fn parse_yaml() -> Result<Vec<Server>, anyhow::Error> {
    let file = File::open("./src/sources.yml")?;

    // getting all the server names
    let servers: Vec<String> = serde_yaml::from_reader(file)?;
    // println!("{servers :?}");

    // parse the ips into the Server struct
    let res: Vec<_> = servers
        .into_iter()
        .map(|item| Server {
            ip: item,
            // can_connect: true,
            ..Default::default()
        })
        .collect();

    // println!("{res :#?}");
    Ok(res)
}

pub async fn check_active_servers() -> Result<Vec<Server>, anyhow::Error> {
    let servers = parse_yaml()?;

    // try to connect each one and filter out the dead ones
    let futures = servers.into_iter().map(|mut s| async {
        let addr: SocketAddr = s.ip.parse().expect("the address is not valid");
        // let res = TcpStream::connect(addr).await;

        // match res {
        //     Ok(_) => {
        //         s.can_connect = true;
        //     }
        //     Err(_) => {
        //         s.can_connect = false;
        //     }
        // }

        // shortcut
        s.can_connect = TcpStream::connect(addr).await.is_ok();

        s
    });

    // run the futures as the async blocks are lazy
    let res = join_all(futures).await;
    Ok(res)
}
