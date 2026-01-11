# backend_core

共有ライブラリクレート。API/CLIで共通のビジネスロジックを提供。

## 機能

- OpenAI Responses API クライアント
- データベース操作（セッション・メッセージ管理）
- 共通モデル・エラー型

## 使用例

```rust
use backend_core::{Config, OpenAIService, SessionRepository};

// OpenAI API
let openai = OpenAIService::new(api_key);
let response = openai.chat(ChatRequest {
    message: "Hello!".to_string(),
    system_prompt: Some("You are helpful.".to_string()),
}).await?;

// セッション管理
let repo = SessionRepository::new(pool);
let session = repo.create_session(Some("System prompt")).await?;
```

## モジュール構成

```
src/
├── lib.rs           # 再エクスポート
├── config.rs        # 設定管理
├── error.rs         # 共通エラー型
├── models/          # 型定義
│   ├── chat.rs      # ChatRequest, ChatResponse
│   └── session.rs   # Session, ChatMessage
├── services/
│   └── openai.rs    # OpenAI API クライアント
└── db/
    ├── repository.rs   # SessionRepository
    └── migrations/     # sqlx migrations
```

## 依存クレート

- `sqlx` - データベース操作
- `reqwest` - HTTP クライアント
- `serde` - シリアライズ
- `thiserror` - エラー定義
