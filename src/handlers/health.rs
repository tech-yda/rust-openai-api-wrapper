use axum::Json;
use serde::Serialize;

/// ヘルスチェックのレスポンス型
/// `#[derive(Serialize)]` で自動的にJSONに変換可能になる
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

/// GET /health ハンドラー
/// `async fn` で非同期関数を定義（TypeScriptの async function 相当）
/// 戻り値 `Json<T>` は自動的にContent-Type: application/jsonになる
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
