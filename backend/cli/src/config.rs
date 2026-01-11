//! 設定ファイル管理

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// CLI設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub default: DefaultConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultConfig {
    #[serde(default = "default_system_prompt")]
    pub system_prompt: String,
    #[serde(default = "default_model")]
    pub model: String,
}

fn default_system_prompt() -> String {
    "You are a helpful assistant.".to_string()
}

fn default_model() -> String {
    "gpt-4o-mini".to_string()
}

impl Default for DefaultConfig {
    fn default() -> Self {
        Self {
            system_prompt: default_system_prompt(),
            model: default_model(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default: DefaultConfig::default(),
        }
    }
}

impl Config {
    /// 設定ファイルを読み込む（なければデフォルト値）
    pub fn load() -> Self {
        let config_path = Self::config_path();
        if config_path.exists() {
            match fs::read_to_string(&config_path) {
                Ok(content) => match toml::from_str(&content) {
                    Ok(config) => return config,
                    Err(e) => {
                        eprintln!("Warning: Failed to parse config file: {}", e);
                    }
                },
                Err(e) => {
                    eprintln!("Warning: Failed to read config file: {}", e);
                }
            }
        }
        Self::default()
    }

    /// 設定ファイルのパスを取得
    pub fn config_path() -> PathBuf {
        Self::config_dir().join("config.toml")
    }

    /// 設定ディレクトリを取得
    pub fn config_dir() -> PathBuf {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "chat-cli") {
            proj_dirs.config_dir().to_path_buf()
        } else {
            PathBuf::from(".chat-cli")
        }
    }

    /// セッション保存ディレクトリを取得
    pub fn sessions_dir() -> PathBuf {
        Self::config_dir().join("sessions")
    }

    /// 履歴保存ディレクトリを取得
    pub fn history_dir() -> PathBuf {
        Self::config_dir().join("history")
    }

    /// 必要なディレクトリを作成
    pub fn ensure_dirs() -> std::io::Result<()> {
        fs::create_dir_all(Self::config_dir())?;
        fs::create_dir_all(Self::sessions_dir())?;
        fs::create_dir_all(Self::history_dir())?;
        Ok(())
    }
}
