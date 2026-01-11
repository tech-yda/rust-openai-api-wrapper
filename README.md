# Rust OpenAI API Wrapper

RustでOpenAI APIをラップするシンプルなAPIサーバー。

## 概要

Rust学習を目的としたプロジェクト。頻出パッケージ（axum, tokio, reqwest, serde, sqlx等）を使用してOpenAI Responses APIのラッパーを実装。

**主な機能:**
- 単発チャット（システムプロンプト対応）
- セッション管理（会話履歴の永続化）
- PostgreSQLによるデータ永続化

## 必要条件

- Rust 1.75+
- Docker & Docker Compose
- OpenAI API Key

## セットアップ

```bash
# リポジトリをクローン
git clone https://github.com/tech-yda/rust-openai-api-wrapper.git
cd rust-openai-api-wrapper

# 環境変数を設定
cp .env.example .env
# .env を編集して OPENAI_API_KEY を設定

# データベースを起動してサーバーを実行
make dev
```

## API エンドポイント

### GET /health
ヘルスチェック

```bash
curl http://localhost:3000/health
```

レスポンス:
```json
{"status":"ok","version":"0.1.0"}
```

### POST /chat
単発チャット（OpenAI Responses API）

```bash
curl -X POST http://localhost:3000/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello!", "system_prompt": "You are a helpful assistant."}'
```

レスポンス:
```json
{
  "response": "Hello! How can I help you today?",
  "model": "gpt-5.2-chat-latest",
  "usage": {
    "prompt_tokens": 20,
    "completion_tokens": 10,
    "total_tokens": 30
  }
}
```

### POST /sessions
新規セッション作成

```bash
curl -X POST http://localhost:3000/sessions \
  -H "Content-Type: application/json" \
  -d '{"system_prompt": "You are a helpful assistant. Always respond in Japanese."}'
```

レスポンス:
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "system_prompt": "You are a helpful assistant. Always respond in Japanese.",
  "created_at": "2026-01-11T12:00:00Z"
}
```

### GET /sessions/{id}
セッション情報取得（メッセージ履歴付き）

```bash
curl http://localhost:3000/sessions/550e8400-e29b-41d4-a716-446655440000
```

レスポンス:
```json
{
  "session": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "system_prompt": "You are a helpful assistant.",
    "created_at": "2026-01-11T12:00:00Z",
    "updated_at": "2026-01-11T12:05:00Z"
  },
  "messages": [
    {"id": "...", "role": "user", "content": "Hello!", "created_at": "..."},
    {"id": "...", "role": "assistant", "content": "Hi there!", "created_at": "..."}
  ]
}
```

### POST /sessions/{id}/chat
セッション内でチャット（履歴を保持）

```bash
curl -X POST http://localhost:3000/sessions/550e8400-e29b-41d4-a716-446655440000/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "What did I just say?"}'
```

レスポンス:
```json
{
  "response": "You said 'Hello!'",
  "model": "gpt-5.2-chat-latest",
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "message_count": 4
}
```

### DELETE /sessions/{id}
セッション削除

```bash
curl -X DELETE http://localhost:3000/sessions/550e8400-e29b-41d4-a716-446655440000
```

レスポンス: `204 No Content`

## プロジェクト構造

```
src/
├── main.rs          # エントリーポイント
├── config.rs        # 設定管理
├── db/              # データベース層
│   ├── mod.rs
│   └── repository.rs
├── handlers/        # HTTPハンドラー
│   ├── mod.rs
│   ├── chat.rs
│   └── session.rs
├── models/          # 型定義
│   ├── mod.rs
│   ├── chat.rs
│   └── session.rs
└── services/        # ビジネスロジック
    ├── mod.rs
    └── openai.rs

migrations/          # SQLマイグレーション
docs/ADR/            # 設計決定記録
```

## 開発コマンド

```bash
# データベース起動 + サーバー実行（推奨）
make dev

# データベースのみ起動
make db

# サーバーのみ実行
cargo run

# ビルド
cargo build

# テスト
cargo test

# Lint
cargo clippy

# フォーマット
cargo fmt

# クリーンアップ（コンテナ・ボリューム削除）
make clean
```

## 技術スタック

| カテゴリ | 技術 |
|---------|------|
| Web Framework | axum |
| Async Runtime | tokio |
| HTTP Client | reqwest |
| Serialization | serde, serde_json |
| Database | PostgreSQL + sqlx |
| Container | Docker, Docker Compose |
| AI API | OpenAI Responses API (GPT-5.2) |

## 設計決定

詳細は [docs/ADR/](./docs/ADR/) を参照。

- [ADR-0005: エラーハンドリング戦略](./docs/ADR/0005-error-handling-strategy.md)
- [ADR-0006: テスト戦略](./docs/ADR/0006-testing-strategy.md)
- [ADR-0007: システムプロンプト戦略](./docs/ADR/0007-system-prompt-strategy.md)

## ライセンス

MIT
