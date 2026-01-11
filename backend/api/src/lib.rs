//! api - Webサーバークレート
//!
//! Axum を使用したHTTP APIサーバーを提供。

pub mod error;
pub mod handlers;

use axum::{
    routing::{delete, get, post},
    Router,
};
use handlers::AppState;

/// アプリケーションのルーターを構築
/// テストから利用可能にするために公開
pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(handlers::health_check))
        .route("/chat", post(handlers::chat))
        .route("/sessions", post(handlers::create_session))
        .route("/sessions/{id}", get(handlers::get_session))
        .route("/sessions/{id}", delete(handlers::delete_session))
        .route("/sessions/{id}/chat", post(handlers::session_chat))
        .with_state(state)
}

async fn root() -> &'static str {
    "Hello, Rust!"
}
