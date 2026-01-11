use axum::{extract::State, Json};
use tracing::info;

use crate::error::AppError;
use crate::handlers::AppState;
use crate::models::{ChatRequest, ChatResponse};

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
