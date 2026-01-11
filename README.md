# Rust OpenAI API Wrapper

RustでOpenAI APIをラップするシンプルなAPIサーバー。

## 概要

Rust学習を目的としたプロジェクト。頻出パッケージ（axum, tokio, reqwest, serde等）を使用してOpenAI Chat Completions APIのラッパーを実装。

## 必要条件

- Rust 1.75+
- OpenAI API Key

## セットアップ

```bash
# リポジトリをクローン
git clone <repository-url>
cd rust-openai-api-wrapper

# 環境変数を設定
cp .env.example .env
# .env を編集して OPENAI_API_KEY を設定

# 起動
cargo run
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
OpenAI Chat Completions APIを呼び出す

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

## プロジェクト構造

```
src/
├── main.rs          # エントリーポイント
├── config.rs        # 設定管理
├── handlers/        # HTTPハンドラー
├── models/          # 型定義
└── services/        # ビジネスロジック
```

## 開発コマンド

```bash
# ビルド
cargo build

# 実行
cargo run

# テスト
cargo test

# Lint
cargo clippy

# フォーマット
cargo fmt
```

## 設計決定

詳細は [docs/ADR/](./docs/ADR/) を参照。

## ライセンス

MIT
