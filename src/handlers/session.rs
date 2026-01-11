use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use tracing::info;
use uuid::Uuid;

use crate::db::SessionRepository;
use crate::error::AppError;
use crate::models::{
    CreateSessionRequest, CreateSessionResponse, Message, SessionChatRequest, SessionChatResponse,
    SessionWithMessages,
};
use crate::services::OpenAIService;

/// アプリケーション共有状態
#[derive(Clone)]
pub struct AppState {
    pub openai: OpenAIService,
    pub session_repo: SessionRepository,
}

/// POST /sessions - 新規セッション作成
pub async fn create_session(
    State(state): State<AppState>,
    Json(request): Json<CreateSessionRequest>,
) -> Result<Json<CreateSessionResponse>, AppError> {
    info!("Creating new session");

    let session = state
        .session_repo
        .create_session(request.system_prompt)
        .await?;

    info!("Session created: {}", session.id);

    Ok(Json(CreateSessionResponse {
        id: session.id,
        system_prompt: session.system_prompt,
        created_at: session.created_at,
    }))
}

/// GET /sessions/{id} - セッション情報取得（履歴付き）
pub async fn get_session(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<SessionWithMessages>, AppError> {
    info!("Getting session: {}", id);

    let session = state
        .session_repo
        .get_session(id)
        .await?
        .ok_or_else(|| AppError::NotFound("Session".to_string()))?;

    let messages = state.session_repo.get_messages(id).await?;

    Ok(Json(SessionWithMessages { session, messages }))
}

/// POST /sessions/{id}/chat - セッション内チャット
pub async fn session_chat(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<SessionChatRequest>,
) -> Result<Json<SessionChatResponse>, AppError> {
    info!("Session chat: {} - message: {}", id, &request.message);

    // セッションを取得
    let session = state
        .session_repo
        .get_session(id)
        .await?
        .ok_or_else(|| AppError::NotFound("Session".to_string()))?;

    // 過去のメッセージを取得
    let history = state.session_repo.get_messages(id).await?;

    // OpenAI API用のメッセージを構築（システムプロンプトはinstructionsで渡す）
    let mut messages: Vec<Message> = Vec::new();

    // 履歴を追加
    for msg in &history {
        messages.push(Message {
            role: msg.role.clone(),
            content: msg.content.clone(),
        });
    }

    // ユーザーメッセージを追加
    messages.push(Message {
        role: "user".to_string(),
        content: request.message.clone(),
    });

    // OpenAI Responses API呼び出し（システムプロンプトはinstructionsパラメータで渡す）
    let response = state
        .openai
        .chat_with_history(messages, session.system_prompt.clone())
        .await?;

    // ユーザーメッセージをDBに保存
    state
        .session_repo
        .add_message(id, "user", &request.message)
        .await?;

    // アシスタントの返答をDBに保存
    state
        .session_repo
        .add_message(id, "assistant", &response.response)
        .await?;

    // 更新後のメッセージ数を取得
    let updated_messages = state.session_repo.get_messages(id).await?;

    info!(
        "Session chat completed: {} - messages: {}",
        id,
        updated_messages.len()
    );

    Ok(Json(SessionChatResponse {
        response: response.response,
        model: response.model,
        session_id: id,
        message_count: updated_messages.len(),
    }))
}

/// DELETE /sessions/{id} - セッション削除
pub async fn delete_session(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    info!("Deleting session: {}", id);

    let deleted = state.session_repo.delete_session(id).await?;

    if deleted {
        info!("Session deleted: {}", id);
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound("Session".to_string()))
    }
}
