//! 共通エラー型
//!
//! API/CLI共通で使用するエラー型を定義。
//! Axum非依存のため、HTTPレスポンス変換はapiクレートで行う。

use thiserror::Error;

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

impl AppError {
    /// エラーコードを取得
    pub fn code(&self) -> &'static str {
        match self {
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::Validation(_) => "VALIDATION_ERROR",
            AppError::Database(_) => "DATABASE_ERROR",
            AppError::ExternalApi(_) => "EXTERNAL_API_ERROR",
        }
    }

    /// ユーザー向けメッセージを取得
    pub fn user_message(&self) -> String {
        match self {
            AppError::NotFound(resource) => format!("{} not found", resource),
            AppError::Validation(msg) => msg.clone(),
            AppError::Database(_) => "Database operation failed".to_string(),
            AppError::ExternalApi(_) => "External service unavailable".to_string(),
        }
    }
}
