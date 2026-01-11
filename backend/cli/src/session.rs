//! セッション管理

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::config::Config;

/// チャットセッション
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub name: String,
    pub system_prompt: String,
    pub messages: Vec<Message>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// チャットメッセージ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Session {
    /// 新しいセッションを作成
    pub fn new(name: Option<String>, system_prompt: String) -> Self {
        let now = Utc::now();
        Self {
            name: name.unwrap_or_else(|| format!("session-{}", now.format("%Y%m%d-%H%M%S"))),
            system_prompt,
            messages: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// メッセージを追加
    pub fn add_message(&mut self, role: &str, content: &str) {
        self.messages.push(Message {
            role: role.to_string(),
            content: content.to_string(),
        });
        self.updated_at = Utc::now();
    }

    /// セッションをクリア（メッセージのみ）
    pub fn clear(&mut self) {
        self.messages.clear();
        self.updated_at = Utc::now();
    }

    /// セッションをファイルに保存
    pub fn save(&self) -> Result<PathBuf, std::io::Error> {
        let path = Self::session_path(&self.name);
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        fs::write(&path, content)?;
        Ok(path)
    }

    /// セッションをファイルから読み込む
    pub fn load(name: &str) -> Result<Self, std::io::Error> {
        let path = Self::session_path(name);
        let content = fs::read_to_string(&path)?;
        serde_json::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    /// セッションを削除
    pub fn delete(name: &str) -> Result<(), std::io::Error> {
        let path = Self::session_path(name);
        fs::remove_file(path)
    }

    /// セッションファイルのパスを取得
    fn session_path(name: &str) -> PathBuf {
        Config::sessions_dir().join(format!("{}.json", name))
    }

    /// OpenAI API用のメッセージ形式に変換
    pub fn to_api_messages(&self) -> Vec<backend_core::models::Message> {
        self.messages
            .iter()
            .map(|msg| backend_core::models::Message {
                role: msg.role.clone(),
                content: msg.content.clone(),
            })
            .collect()
    }

    /// システムプロンプトを取得
    pub fn system_prompt(&self) -> Option<String> {
        if self.system_prompt.is_empty() {
            None
        } else {
            Some(self.system_prompt.clone())
        }
    }
}

/// 保存済みセッション一覧を取得
pub fn list_sessions() -> Result<Vec<SessionSummary>, std::io::Error> {
    let sessions_dir = Config::sessions_dir();
    if !sessions_dir.exists() {
        return Ok(Vec::new());
    }

    let mut sessions = Vec::new();
    for entry in fs::read_dir(sessions_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "json") {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(session) = serde_json::from_str::<Session>(&content) {
                    sessions.push(SessionSummary {
                        name: session.name,
                        message_count: session.messages.len(),
                        updated_at: session.updated_at,
                    });
                }
            }
        }
    }

    sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(sessions)
}

/// セッションサマリー
#[derive(Debug)]
pub struct SessionSummary {
    pub name: String,
    pub message_count: usize,
    pub updated_at: DateTime<Utc>,
}
