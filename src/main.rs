use laminar::common::*;

#[tokio::main]
async fn main() {
    let res = check_active_servers().await.expect(" wah bete moj krdi ");
    println!("{res :#?}");
}
