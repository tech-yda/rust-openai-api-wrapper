# ADR-0005: エラーハンドリング戦略

## ステータス
採用

## 日付
2026-01-11

## コンテキスト
現状、`SessionError`と`AppError`の2つのエラー型が別々に定義されており、エラーレスポンスのフォーマットも統一されていない。業務レベルのWebアプリケーションとして、一貫したエラーハンドリングが必要。

### 現状の問題
- エラー型が分散している（handlers/session.rs, handlers/chat.rs）
- レスポンス形式が `{"error": "msg"}` と単純
- 内部エラーの詳細がそのままクライアントに返る可能性

## 決定

### エラー型の統一

`thiserror`を使用した統一エラー型`AppError`を作成する。

```rust
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Database error")]
    Database(#[from] sqlx::Error),

    #[error("External API error")]
    ExternalApi(#[from] reqwest::Error),

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}
```

### エラーレスポンス形式

構造化JSONレスポンスを採用。

```json
{
  "error": {
    "code": "NOT_FOUND",
    "message": "Session 123 not found"
  }
}
```

### セキュリティ考慮

| エラー種別 | ログ | クライアントレスポンス |
|-----------|------|----------------------|
| NotFound | 不要 | そのまま |
| Validation | 不要 | そのまま |
| Database | 詳細を記録 | 汎用メッセージ |
| ExternalApi | 詳細を記録 | 汎用メッセージ |
| Internal | 詳細を記録 | 汎用メッセージ |

### HTTPステータスコードマッピング

| AppError | HTTPステータス |
|----------|---------------|
| NotFound | 404 |
| Validation | 400 |
| Database | 500 |
| ExternalApi | 502 |
| Internal | 500 |

## 検討したが採用しなかったもの

| 選択肢 | 理由 |
|--------|------|
| anyhowのみ使用 | 型情報が失われ、マッチングが困難 |
| エラー型を分離したまま | コードの重複、一貫性の欠如 |
| 詳細エラーをそのまま返す | セキュリティリスク |

## 参考資料
- [Axum Error Handling - LogRocket](https://blog.logrocket.com/rust-axum-error-handling/)
- [Elegant Error Handling with IntoResponse - Leapcell](https://leapcell.io/blog/elegant-error-handling-in-axum-actix-web-with-intoresponse)
- [Rust Error Handling 2025 Guide](https://markaicode.com/rust-error-handling-2025-guide/)

## 結果
- 全ハンドラーで統一されたエラー型を使用
- `#[from]`による自動変換で`?`演算子が使いやすい
- 構造化レスポンスでクライアント側の処理が容易
- セキュリティを考慮したエラーメッセージ
