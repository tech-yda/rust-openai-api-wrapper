//! API用エラーハンドリング
//!
//! core::AppErrorをAxumのHTTPレスポンスに変換する。
//! Orphan ruleを回避するためにnewtypeパターンを使用。

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use backend_core::AppError;
use backend_core::services::OpenAIError;
use tracing::error;

/// API用エラー型（core::AppErrorのラッパー）
pub struct ApiError(pub AppError);

impl From<AppError> for ApiError {
    fn from(err: AppError) -> Self {
        ApiError(err)
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        ApiError(AppError::Database(err))
    }
}

impl From<OpenAIError> for ApiError {
    fn from(err: OpenAIError) -> Self {
        ApiError(AppError::ExternalApi(err))
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let inner = self.0;

        // ステータスコードを決定
        let status = match &inner {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::Database(e) => {
                error!("Database error: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            AppError::ExternalApi(e) => {
                error!("External API error: {:?}", e);
                StatusCode::BAD_GATEWAY
            }
        };

        // 構造化されたエラーレスポンス
        let body = serde_json::json!({
            "error": {
                "code": inner.code(),
                "message": inner.user_message()
            }
        });

        (status, Json(body)).into_response()
    }
}
