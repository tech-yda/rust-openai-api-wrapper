use axum::{extract::State, http::StatusCode, response::{IntoResponse, Response}, Json};
use tracing::{error, info};

use crate::handlers::AppState;
use crate::models::{ChatRequest, ChatResponse};
use crate::services::OpenAIError;

/// アプリケーションエラー型
pub struct AppError(OpenAIError);

impl From<OpenAIError> for AppError {
    fn from(error: OpenAIError) -> Self {
        AppError(error)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self.0 {
            OpenAIError::RequestError(e) => {
                error!("Request error: {}", e);
                (StatusCode::BAD_GATEWAY, "Failed to connect to OpenAI API")
            }
            OpenAIError::ApiError(msg) => {
                error!("OpenAI API error: {}", msg);
                (StatusCode::BAD_REQUEST, "OpenAI API returned an error")
            }
        };

        let body = serde_json::json!({ "error": message });
        (status, Json(body)).into_response()
    }
}

/// POST /chat ハンドラー
pub async fn chat(
    State(state): State<AppState>,
    Json(request): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, AppError> {
    info!("Chat request received");

    let response = state.openai.chat(request).await?;

    info!(
        "Chat response sent (tokens: {})",
        response.usage.total_tokens
    );
    Ok(Json(response))
}
