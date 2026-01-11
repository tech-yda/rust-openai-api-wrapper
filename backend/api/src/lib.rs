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
use tower_http::cors::{Any, CorsLayer};

/// アプリケーションのルーターを構築
/// テストから利用可能にするために公開
pub fn create_app(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/", get(root))
        .route("/health", get(handlers::health_check))
        .route("/chat", post(handlers::chat))
        .route("/sessions", post(handlers::create_session))
        .route("/sessions/{id}", get(handlers::get_session))
        .route("/sessions/{id}", delete(handlers::delete_session))
        .route("/sessions/{id}/chat", post(handlers::session_chat))
        .layer(cors)
        .with_state(state)
}

async fn root() -> &'static str {
    "Hello, Rust!"
}
