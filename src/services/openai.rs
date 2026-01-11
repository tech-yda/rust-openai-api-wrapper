use reqwest::Client;
use thiserror::Error;

use crate::models::{ChatRequest, ChatResponse, Message, OpenAIRequest, OpenAIResponse, Usage};

/// OpenAI API のエンドポイント
const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";
/// 使用するモデル（2026年1月最新）
const MODEL: &str = "gpt-5.2-chat-latest";

/// OpenAI サービスのエラー型
/// `#[derive(Error)]` で std::error::Error を自動実装
/// `#[error("...")]` でエラーメッセージを定義
#[derive(Error, Debug)]
pub enum OpenAIError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("OpenAI API error: {0}")]
    ApiError(String),
}

/// OpenAI API クライアント
/// `Clone` を derive することで、複数のハンドラーで共有可能
#[derive(Clone)]
pub struct OpenAIService {
    client: Client,
    api_key: String,
}

impl OpenAIService {
    /// 新しいクライアントを作成
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    /// Chat Completions API を呼び出す
    pub async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, OpenAIError> {
        // メッセージを構築
        let mut messages = Vec::new();

        // system_prompt があれば追加
        if let Some(system) = request.system_prompt {
            messages.push(Message {
                role: "system".to_string(),
                content: system,
            });
        }

        // ユーザーメッセージを追加
        messages.push(Message {
            role: "user".to_string(),
            content: request.message,
        });

        // OpenAI API リクエストを構築
        let openai_request = OpenAIRequest {
            model: MODEL.to_string(),
            messages,
        };

        // API を呼び出し
        let response = self
            .client
            .post(OPENAI_API_URL)
            .bearer_auth(&self.api_key)
            .json(&openai_request)
            .send()
            .await?;

        // ステータスコードをチェック
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(OpenAIError::ApiError(error_text));
        }

        // レスポンスをパース
        let openai_response: OpenAIResponse = response.json().await?;

        // レスポンスを変換
        // `.first()` は最初の要素を Option で返す
        let assistant_message = openai_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        Ok(ChatResponse {
            response: assistant_message,
            model: openai_response.model,
            usage: Usage {
                prompt_tokens: openai_response.usage.prompt_tokens,
                completion_tokens: openai_response.usage.completion_tokens,
                total_tokens: openai_response.usage.total_tokens,
            },
        })
    }
}
