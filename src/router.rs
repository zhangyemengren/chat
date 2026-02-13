use axum::{Router, http::StatusCode, response::{IntoResponse, Sse}, routing, extract::State, Json};
use tower_http::services::ServeFile;
use crate::config::Config;
use crate::openrouter::{fetch_chat_sse, ChatRequest};

pub async fn chat_handler(
    State(config): State<Config>,
    Json(req): Json<ChatRequest>,
) -> impl IntoResponse {
    match fetch_chat_sse(&config, req).await {
        Ok(stream) => Sse::new(stream).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub fn router() -> Router {
    let config = Config::from_env().unwrap();
    Router::new()
        .route_service("/", ServeFile::new("assets/index.html"))
        .route("/chat", routing::post(chat_handler))
        .with_state(config)

}
