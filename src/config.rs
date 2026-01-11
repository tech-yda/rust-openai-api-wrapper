use std::env;

/// アプリケーション設定
/// `#[derive(Clone)]` で値のコピーが可能になる（所有権を移動せずに共有できる）
#[derive(Clone)]
pub struct Config {
    pub openai_api_key: String,
    pub host: String,
    pub port: u16,
}

impl Config {
    /// 環境変数から設定を読み込む
    /// `Result<Self, String>` は成功時にConfig、失敗時にエラーメッセージを返す
    pub fn from_env() -> Result<Self, String> {
        // dotenvy::dotenv() は .env ファイルを読み込む（失敗しても続行）
        let _ = dotenvy::dotenv();

        // env::var() は環境変数を取得、なければ Err を返す
        let openai_api_key = env::var("OPENAI_API_KEY").map_err(|_| "OPENAI_API_KEY is not set")?;

        // unwrap_or_else: 値がなければデフォルト値を使用
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

        // parse::<u16>() で文字列を数値に変換
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .map_err(|_| "PORT must be a valid number")?;

        Ok(Self {
            openai_api_key,
            host,
            port,
        })
    }

    /// サーバーアドレスを取得
    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
