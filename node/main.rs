mod config;

#[tokio::main]
async fn main() {
    println!("Starting Nythera Node...");

    let config = config::load();
    println!("Network: {}", config.chain_name);
}
