//! 統合テスト
//!
//! テスト実行には PostgreSQL が必要です:
//! ```bash
//! make db  # または docker-compose up -d db
//! cargo test -p api
//! ```

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use api::{create_app, handlers::AppState};
use backend_core::{OpenAIService, SessionRepository};
use serde_json::{json, Value};
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;

/// テスト用のデータベースURLを取得
fn get_test_database_url() -> String {
    std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5433/chat_app".to_string())
}

/// テスト用のAppStateを作成
async fn create_test_state() -> Option<AppState> {
    // .envファイルを読み込み
    let _ = dotenvy::dotenv();

    let database_url = get_test_database_url();

    // データベースに接続を試みる
    let pool = match PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await
    {
        Ok(pool) => pool,
        Err(_) => {
            eprintln!("Warning: Could not connect to database. Skipping integration tests.");
            return None;
        }
    };

    // マイグレーション実行
    if sqlx::migrate!("../core/src/db/migrations").run(&pool).await.is_err() {
        eprintln!("Warning: Could not run migrations. Skipping integration tests.");
        return None;
    }

    // ダミーのOpenAI APIキー（テストでは実際のAPIは呼ばない）
    let api_key =
        std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| "test-api-key".to_string());

    let openai_service = OpenAIService::new(api_key);
    let session_repo = SessionRepository::new(pool);

    Some(AppState {
        openai: openai_service,
        session_repo,
    })
}

// ============================================
// ヘルスチェックテスト（DBなしでも動作）
// ============================================

#[tokio::test]
async fn test_health_check() {
    let state = match create_test_state().await {
        Some(s) => s,
        None => return, // DB接続できない場合はスキップ
    };

    let app = create_app(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["status"], "ok");
    assert!(json["version"].is_string());
}

#[tokio::test]
async fn test_root() {
    let state = match create_test_state().await {
        Some(s) => s,
        None => return,
    };

    let app = create_app(state);

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"Hello, Rust!");
}

// ============================================
// セッション管理テスト
// ============================================

#[tokio::test]
async fn test_create_session() {
    let state = match create_test_state().await {
        Some(s) => s,
        None => return,
    };

    let app = create_app(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/sessions")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({"system_prompt": "You are a test assistant."}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json["id"].is_string());
    assert_eq!(json["system_prompt"], "You are a test assistant.");
    assert!(json["created_at"].is_string());
}

#[tokio::test]
async fn test_create_session_without_system_prompt() {
    let state = match create_test_state().await {
        Some(s) => s,
        None => return,
    };

    let app = create_app(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/sessions")
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json["id"].is_string());
    assert!(json["system_prompt"].is_null());
}

#[tokio::test]
async fn test_get_session_not_found() {
    let state = match create_test_state().await {
        Some(s) => s,
        None => return,
    };

    let app = create_app(state);

    // 存在しないUUIDでリクエスト
    let response = app
        .oneshot(
            Request::builder()
                .uri("/sessions/00000000-0000-0000-0000-000000000000")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // 構造化されたエラーレスポンスを検証
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["error"]["code"], "NOT_FOUND");
    assert!(json["error"]["message"].as_str().unwrap().contains("not found"));
}

#[tokio::test]
async fn test_delete_session_not_found() {
    let state = match create_test_state().await {
        Some(s) => s,
        None => return,
    };

    let app = create_app(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/sessions/00000000-0000-0000-0000-000000000000")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// ============================================
// セッションCRUDフローテスト
// ============================================

#[tokio::test]
async fn test_session_crud_flow() {
    let state = match create_test_state().await {
        Some(s) => s,
        None => return,
    };

    // 1. セッション作成
    let app = create_app(state.clone());
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/sessions")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({"system_prompt": "CRUD test session"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let create_json: Value = serde_json::from_slice(&body).unwrap();
    let session_id = create_json["id"].as_str().unwrap();

    // 2. セッション取得
    let app = create_app(state.clone());
    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/sessions/{}", session_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let get_json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(get_json["session"]["id"], session_id);
    assert_eq!(get_json["session"]["system_prompt"], "CRUD test session");
    assert!(get_json["messages"].as_array().unwrap().is_empty());

    // 3. セッション削除
    let app = create_app(state.clone());
    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/sessions/{}", session_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // 4. 削除後の取得は404
    let app = create_app(state);
    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/sessions/{}", session_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
