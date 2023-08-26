use std::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = std::env::args();
    let ip_addr = "127.0.0.1";
    let default_port = 8080;
    let port = args
        .nth(1)
        .map(|s| s.parse().unwrap_or(default_port))
        .unwrap_or(default_port);
    blockchain_rs::server::main(ip_addr, port).await
}
