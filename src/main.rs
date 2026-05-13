use laminar::common::*;

#[tokio::main]
async fn main() {
    let res = check_servers_health().await.expect(" wah bete moj krdi ");
    println!("{res :#?}");
}
