//! core - 共有ライブラリクレート
//!
//! API/CLIで共有するロジックを提供。
//! - 設定管理
//! - OpenAI APIサービス
//! - データベース操作
//! - 共通モデル・エラー型

pub mod config;
pub mod db;
pub mod error;
pub mod models;
pub mod services;

// 主要な型を再エクスポート
pub use config::Config;
pub use db::SessionRepository;
pub use error::AppError;
pub use services::OpenAIService;
