use reqwest::Client;
use thiserror::Error;

use crate::models::{ChatRequest, ChatResponse, Message, OpenAIRequest, OpenAIResponse, Usage};

/// OpenAI Responses API のエンドポイント
const OPENAI_API_URL: &str = "https://api.openai.com/v1/responses";
/// 使用するモデル（GPT-5.2 Instant）
const MODEL: &str = "gpt-5.2-chat-latest";

/// OpenAI サービスのエラー型
#[derive(Error, Debug)]
pub enum OpenAIError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("OpenAI API error: {0}")]
    ApiError(String),
}

/// OpenAI API クライアント
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

    /// Responses API を呼び出す（単発チャット）
    pub async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, OpenAIError> {
        let input = vec![Message {
            role: "user".to_string(),
            content: request.message,
        }];

        self.call_responses_api(input, request.system_prompt).await
    }

    /// 履歴を含めた Responses API を呼び出す
    pub async fn chat_with_history(
        &self,
        messages: Vec<Message>,
        instructions: Option<String>,
    ) -> Result<ChatResponse, OpenAIError> {
        self.call_responses_api(messages, instructions).await
    }

    /// Responses API を呼び出す（内部メソッド）
    async fn call_responses_api(
        &self,
        input: Vec<Message>,
        instructions: Option<String>,
    ) -> Result<ChatResponse, OpenAIError> {
        // Responses API リクエストを構築
        let openai_request = OpenAIRequest {
            model: MODEL.to_string(),
            input,
            instructions,
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

        // outputから"message"タイプのテキストを抽出
        // 注意: "reasoning"（内部思考）は使用しない - "message"（公開出力）のみを使用
        let response_text = openai_response
            .output
            .iter()
            .filter(|item| item.item_type == "message")
            .find_map(|item| {
                item.content.first().and_then(|c| c.text.clone())
            })
            .unwrap_or_default();

        // レスポンスを変換
        Ok(ChatResponse {
            response: response_text,
            model: openai_response.model,
            usage: Usage {
                prompt_tokens: openai_response.usage.input_tokens,
                completion_tokens: openai_response.usage.output_tokens,
                total_tokens: openai_response.usage.total_tokens,
            },
        })
    }
}
