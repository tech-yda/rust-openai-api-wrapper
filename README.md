# Chat App

OpenAI API を使ったチャットアプリケーション。Web UI、REST API、CLI の3つのインターフェースを提供。

## 機能

- Web UI（TanStack Start + shadcn/ui）
- REST API（Axum）
- CLI（インタラクティブREPL）
- セッション管理・会話履歴の永続化

## クイックスタート

```bash
# 環境変数を設定
cp .env.example .env
# .env を編集して OPENAI_API_KEY を設定

# 全サービス起動（DB + API + UI）
make dev

# アクセス
# - UI: http://localhost:3000
# - API: http://localhost:8080
```

## CLI 使用

```bash
# ワンショット質問
cargo run -p cli -- ask "Hello"

# インタラクティブチャット
cargo run -p cli -- chat
```

## プロジェクト構成

```
├── backend/
│   ├── core/     # 共有ライブラリ（モデル、サービス、DB）
│   ├── api/      # REST API サーバー
│   └── cli/      # CLI ツール
├── ui/           # Web フロントエンド
├── migrations/   # SQLマイグレーション
└── docs/adr/     # 設計決定記録
```

詳細は各ディレクトリの README を参照：
- [backend/core/README.md](./backend/core/README.md)
- [backend/api/README.md](./backend/api/README.md)
- [backend/cli/README.md](./backend/cli/README.md)

## 開発コマンド

```bash
make dev       # 全サービス起動
make dev-api   # API + DB のみ
make down      # 停止
make clean     # クリーンアップ（ボリューム含む）
make test      # テスト実行
make lint      # Clippy
make fmt       # フォーマット
```

## 技術スタック

| カテゴリ | 技術 |
|---------|------|
| Backend | Rust, Axum, sqlx |
| Frontend | TanStack Start, shadcn/ui, Tailwind CSS |
| Database | PostgreSQL |
| AI | OpenAI Responses API |
| Dev | Docker Compose |

## ライセンス

MIT
