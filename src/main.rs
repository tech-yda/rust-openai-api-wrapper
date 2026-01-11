// モジュール宣言
mod config;
mod handlers;
mod models;
mod services;

use axum::{
    Router,
    routing::{get, post},
};
use config::Config;
use services::OpenAIService;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // ロギング初期化
    // RUST_LOG 環境変数でログレベルを制御（例: RUST_LOG=debug）
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_openai_api_wrapper=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 設定を読み込む
    let config = Config::from_env().expect("Failed to load config");

    // OpenAI サービスを初期化
    let openai_service = OpenAIService::new(config.openai_api_key.clone());

    // ルーター設定
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(handlers::health_check))
        .route("/chat", post(handlers::chat))
        .with_state(openai_service);

    let addr = config.server_addr();
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    info!("Server running at http://{}", addr);
    info!("Endpoints:");
    info!("  GET  /       - Hello message");
    info!("  GET  /health - Health check");
    info!("  POST /chat   - Chat with OpenAI");

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, Rust!"
}
