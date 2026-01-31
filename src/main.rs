mod config;
mod openrouter;
mod router;

use router::router;

#[tokio::main]
async fn main() {
    
    // fetch_chat(&cfg).await.unwrap();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router()).await.unwrap();
}
