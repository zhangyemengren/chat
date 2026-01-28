mod config;
mod fetch;

use config::Config;


#[tokio::main]
async fn main() {
    let cfg = Config::from_env().unwrap();
    println!("Loaded config: api_key length = {}", cfg.api_key.len());
}
