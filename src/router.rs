use axum::{
    Router,
    http::StatusCode,
    response::{IntoResponse, Sse},
    routing,
    extract::State,
};
use axum::response::sse::KeepAlive;
use tower_http::services::ServeFile;
use crate::config::Config;
use crate::openrouter::fetch_chat_sse;

pub async fn chat_handler(State(config): State<Config>) -> impl IntoResponse {
    match fetch_chat_sse(&config).await {
        Ok(stream) => Sse::new(stream).keep_alive(KeepAlive::default()).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub fn router() -> Router {
    let config = Config::from_env().unwrap();
    Router::new()
        .route_service("/", ServeFile::new("assets/index.html"))
        .route("/chat", routing::get(chat_handler))
        .with_state(config)

}
