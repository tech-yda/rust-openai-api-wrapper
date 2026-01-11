use std::env;

/// アプリケーション設定
#[derive(Clone)]
pub struct Config {
    pub openai_api_key: String,
    pub host: String,
    pub port: u16,
    pub database_url: String,
}

impl Config {
    /// 環境変数から設定を読み込む
    pub fn from_env() -> Result<Self, String> {
        let _ = dotenvy::dotenv();

        let openai_api_key =
            env::var("OPENAI_API_KEY").map_err(|_| "OPENAI_API_KEY is not set")?;

        let database_url = env::var("DATABASE_URL").map_err(|_| "DATABASE_URL is not set")?;

        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .map_err(|_| "PORT must be a valid number")?;

        Ok(Self {
            openai_api_key,
            host,
            port,
            database_url,
        })
    }

    /// サーバーアドレスを取得
    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
