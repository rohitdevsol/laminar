use std::fs::File;
use anyhow;
use laminar::common::*;

fn main() {
    parse_yaml();
}

pub fn parse_yaml() -> Result<(), anyhow::Error> {
    let file = File::open("./src/sources.yml")?;

    // getting all the server names
    let servers: Vec<String> = serde_yaml::from_reader(file)?;
    println!("{servers :?}");

    // parse the ips into the Server struct
    let res: Vec<_> = servers
        .into_iter()
        .map(|item| Server {
            ip: item,
            // can_connect: true,
            ..Default::default()
        })
        .collect();

    println!("{res :#?}");
    Ok(())
}
