use serde::{Deserialize, Serialize};

// ========================================
// API リクエスト/レスポンス（外部向け）
// ========================================

/// クライアントからのリクエスト
#[derive(Deserialize)]
pub struct ChatRequest {
    pub message: String,
    #[serde(default)] // フィールドがなければデフォルト値（None）を使用
    pub system_prompt: Option<String>,
}

/// クライアントへのレスポンス
#[derive(Serialize)]
pub struct ChatResponse {
    pub response: String,
    pub model: String,
    pub usage: Usage,
}

#[derive(Serialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

// ========================================
// OpenAI API 用の型定義（内部用）
// ========================================

/// OpenAI API へのリクエスト
#[derive(Serialize)]
pub struct OpenAIRequest {
    pub model: String,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// OpenAI API からのレスポンス
#[derive(Deserialize)]
pub struct OpenAIResponse {
    pub choices: Vec<Choice>,
    pub model: String,
    pub usage: OpenAIUsage,
}

#[derive(Deserialize)]
pub struct Choice {
    pub message: Message,
}

#[derive(Deserialize)]
pub struct OpenAIUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
