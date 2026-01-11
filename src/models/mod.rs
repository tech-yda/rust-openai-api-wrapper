// データ構造・型定義

pub mod chat;

// 頻繁に使う型を再エクスポート
pub use chat::{ChatRequest, ChatResponse, Message, OpenAIRequest, OpenAIResponse, Usage};
