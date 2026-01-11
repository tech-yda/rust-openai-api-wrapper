use axum::{extract::State, Json};
use tracing::info;

use backend_core::models::{ChatRequest, ChatResponse};
use crate::error::ApiError;
use crate::handlers::AppState;

/// POST /chat ハンドラー
pub async fn chat(
    State(state): State<AppState>,
    Json(request): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, ApiError> {
    info!("Chat request received");

    let response = state.openai.chat(request).await?;

    info!(
        "Chat response sent (tokens: {})",
        response.usage.total_tokens
    );
    Ok(Json(response))
}
