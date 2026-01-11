// モジュール宣言
mod config;
mod db;
mod handlers;
mod models;
mod services;

use axum::{
    routing::{delete, get, post},
    Router,
};
use config::Config;
use db::SessionRepository;
use handlers::AppState;
use services::OpenAIService;
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // ロギング初期化
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_openai_api_wrapper=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 設定を読み込む
    let config = Config::from_env().expect("Failed to load config");

    // データベース接続プールを作成
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    info!("Connected to database");

    // マイグレーション実行
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    info!("Migrations completed");

    // サービスとリポジトリを初期化
    let openai_service = OpenAIService::new(config.openai_api_key.clone());
    let session_repo = SessionRepository::new(pool);

    // アプリケーション状態
    let app_state = AppState {
        openai: openai_service,
        session_repo,
    };

    // ルーター設定
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(handlers::health_check))
        .route("/chat", post(handlers::chat))
        // セッション関連エンドポイント
        .route("/sessions", post(handlers::create_session))
        .route("/sessions/{id}", get(handlers::get_session))
        .route("/sessions/{id}", delete(handlers::delete_session))
        .route("/sessions/{id}/chat", post(handlers::session_chat))
        .with_state(app_state);

    let addr = config.server_addr();
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    info!("Server running at http://{}", addr);
    info!("Endpoints:");
    info!("  GET    /                  - Hello message");
    info!("  GET    /health            - Health check");
    info!("  POST   /chat              - Chat with OpenAI (single)");
    info!("  POST   /sessions          - Create new session");
    info!("  GET    /sessions/{{id}}     - Get session with messages");
    info!("  DELETE /sessions/{{id}}     - Delete session");
    info!("  POST   /sessions/{{id}}/chat - Chat within session");

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, Rust!"
}
