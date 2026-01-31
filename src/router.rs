use axum::{routing, Router};
use axum::extract::State;
use futures_util::{StreamExt, pin_mut};
use tower_http::services::ServeFile;
use crate::config::Config;
use crate::openrouter::fetch_chat;

pub async fn chat_handler(config: State<Config>) -> &'static str {
    let mut s = fetch_chat(&config).await.unwrap();
    while let Some(_) = s.next().await {
        println!("Received chunk");
    }
    "Chat"
}

pub fn router() -> Router {
    let config = Config::from_env().unwrap();
    Router::new()
        .route_service("/", ServeFile::new("assets/index.html"))
        .route("/chat", routing::post(chat_handler))
        .with_state(config)

}
