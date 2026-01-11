# ADR-0004: Phase 2 - データベース導入計画

## ステータス
計画中

## 日付
2026-01-11

## コンテキスト
Phase 1ではステートレスなAPIサーバーを構築した。Phase 2ではチャット履歴を保持するためにデータベースを導入する。

## 検討中の選択肢

### データベース

| DB | 特徴 | 判断 |
|----|------|------|
| **SQLite** | ファイルベース、セットアップ不要 | 学習用に推奨 |
| PostgreSQL | フル機能、本番向け | 本番デプロイ時に検討 |

### ORMライブラリ

| ライブラリ | 特徴 | 判断 |
|-----------|------|------|
| **sqlx** | コンパイル時SQL検証、async対応 | 推奨 |
| diesel | 型安全ORM、マクロベース | 学習曲線が急 |
| SeaORM | ActiveRecord風、async対応 | sqlxで十分 |

## 予定する追加パッケージ

```toml
[dependencies]
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite"] }
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
```

## 予定するディレクトリ構造

```
src/
├── main.rs
├── config.rs
├── handlers/
├── models/
├── services/
└── db/              # 新規追加
    ├── mod.rs
    ├── migrations/  # スキーマ定義
    └── repository.rs
```

## 予定するスキーマ

```sql
-- チャットセッション
CREATE TABLE sessions (
    id TEXT PRIMARY KEY,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

-- チャット履歴
CREATE TABLE messages (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    FOREIGN KEY (session_id) REFERENCES sessions(id)
);
```

## 予定する新規エンドポイント

| メソッド | パス | 説明 |
|----------|------|------|
| POST | /sessions | 新規セッション作成 |
| GET | /sessions/:id | セッション情報取得 |
| GET | /sessions/:id/messages | 履歴取得 |
| POST | /sessions/:id/chat | セッション内でチャット |

## 次のアクション
- [ ] sqlxのセットアップ
- [ ] マイグレーション作成
- [ ] リポジトリ層の実装
- [ ] 既存エンドポイントの拡張
