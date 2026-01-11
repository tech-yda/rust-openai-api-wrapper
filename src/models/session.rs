use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ========================================
// DB モデル
// ========================================

/// セッション（会話のコンテナ）
#[derive(Debug, FromRow, Serialize)]
pub struct Session {
    pub id: Uuid,
    pub system_prompt: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// メッセージ（会話履歴の1行）
#[derive(Debug, FromRow, Serialize, Clone)]
pub struct ChatMessage {
    pub id: Uuid,
    pub session_id: Uuid,
    pub role: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

// ========================================
// API リクエスト/レスポンス
// ========================================

/// セッション作成リクエスト
#[derive(Deserialize, Default)]
pub struct CreateSessionRequest {
    #[serde(default)]
    pub system_prompt: Option<String>,
}

/// セッション作成レスポンス
#[derive(Serialize)]
pub struct CreateSessionResponse {
    pub id: Uuid,
    pub system_prompt: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// セッション内チャットリクエスト
#[derive(Deserialize)]
pub struct SessionChatRequest {
    pub message: String,
}

/// セッション内チャットレスポンス
#[derive(Serialize)]
pub struct SessionChatResponse {
    pub response: String,
    pub model: String,
    pub session_id: Uuid,
    pub message_count: usize,
}

/// セッション情報（履歴付き）
#[derive(Serialize)]
pub struct SessionWithMessages {
    pub session: Session,
    pub messages: Vec<ChatMessage>,
}
