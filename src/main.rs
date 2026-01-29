mod config;
mod fetch;

use config::Config;
use fetch::fetch_chat;

#[tokio::main]
async fn main() {
    let cfg = Config::from_env().unwrap();
    println!("Loaded config: api_key length = {}", cfg.api_key.len());
    fetch_chat(&cfg).await.unwrap();
}
