# API Server

Axum ベースの REST API サーバー。

## 起動方法

```bash
# Docker Compose（推奨）
make dev

# 直接実行
cargo run -p api
```

## エンドポイント

| Method | Path | 説明 |
|--------|------|------|
| GET | `/` | ルート |
| GET | `/health` | ヘルスチェック |
| POST | `/chat` | 単発チャット |
| POST | `/sessions` | セッション作成 |
| GET | `/sessions/{id}` | セッション取得 |
| DELETE | `/sessions/{id}` | セッション削除 |
| POST | `/sessions/{id}/chat` | セッション内チャット |

## API 使用例

### 単発チャット

```bash
curl -X POST http://localhost:8080/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello!"}'
```

### セッション作成

```bash
curl -X POST http://localhost:8080/sessions \
  -H "Content-Type: application/json" \
  -d '{"system_prompt": "You are a helpful assistant."}'
```

### セッション内チャット

```bash
curl -X POST http://localhost:8080/sessions/{id}/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "What is Rust?"}'
```

## 環境変数

| 変数 | 説明 | デフォルト |
|------|------|-----------|
| `DATABASE_URL` | PostgreSQL接続文字列 | 必須 |
| `OPENAI_API_KEY` | OpenAI APIキー | 必須 |
| `HOST` | バインドアドレス | `0.0.0.0` |
| `PORT` | ポート番号 | `8080` |

## モジュール構成

```
src/
├── main.rs          # エントリーポイント
├── lib.rs           # Router定義
├── error.rs         # Axum用エラー変換
└── handlers/
    ├── chat.rs      # /chat
    └── session.rs   # /sessions
```
