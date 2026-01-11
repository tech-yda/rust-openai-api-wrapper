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
// OpenAI Responses API 用の型定義（内部用）
// ========================================

/// メッセージ（input配列の要素）
#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// OpenAI Responses API へのリクエスト
#[derive(Serialize)]
pub struct OpenAIRequest {
    pub model: String,
    pub input: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
}

/// OpenAI Responses API からのレスポンス
#[derive(Deserialize, Debug)]
pub struct OpenAIResponse {
    pub id: String,
    pub model: String,
    pub output: Vec<OutputItem>,
    pub usage: OpenAIUsage,
}

/// output配列の要素（type: "message" または "reasoning"）
#[derive(Deserialize, Debug)]
pub struct OutputItem {
    /// アイテムの種類: "message"（公開出力）または "reasoning"（内部思考）
    #[serde(rename = "type")]
    pub item_type: String,
    /// コンテンツ（messageタイプのみ存在）
    #[serde(default)]
    pub content: Vec<ContentItem>,
}

/// content配列の要素
#[derive(Deserialize, Debug)]
pub struct ContentItem {
    #[serde(default)]
    pub text: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct OpenAIUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub total_tokens: u32,
}
