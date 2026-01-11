//! 統一エラーハンドリング
//!
//! 全ハンドラーで使用する共通のエラー型を定義。
//! セキュリティのため、内部エラーはログに記録し、ユーザーには汎用メッセージを返す。

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;
use tracing::error;

use crate::services::OpenAIError;

/// アプリケーション全体で使用するエラー型
#[derive(Error, Debug)]
pub enum AppError {
    /// リソースが見つからない
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// バリデーションエラー
    #[error("Validation error: {0}")]
    Validation(String),

    /// データベースエラー
    #[error("Database error")]
    Database(#[from] sqlx::Error),

    /// 外部APIエラー（OpenAI）
    #[error("External API error")]
    ExternalApi(#[from] OpenAIError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // エラーコードとメッセージを決定
        // 注意: 内部詳細はログへ、ユーザーには汎用メッセージを返す
        let (status, code, message) = match &self {
            AppError::NotFound(resource) => (
                StatusCode::NOT_FOUND,
                "NOT_FOUND",
                format!("{} not found", resource),
            ),
            AppError::Validation(msg) => (
                StatusCode::BAD_REQUEST,
                "VALIDATION_ERROR",
                msg.clone(),
            ),
            AppError::Database(e) => {
                error!("Database error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR",
                    "Database operation failed".to_string(),
                )
            }
            AppError::ExternalApi(e) => {
                error!("External API error: {:?}", e);
                (
                    StatusCode::BAD_GATEWAY,
                    "EXTERNAL_API_ERROR",
                    "External service unavailable".to_string(),
                )
            }
        };

        // 構造化されたエラーレスポンス
        let body = serde_json::json!({
            "error": {
                "code": code,
                "message": message
            }
        });

        (status, Json(body)).into_response()
    }
}
