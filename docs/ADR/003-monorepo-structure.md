# ADR-003: モノレポ構成への移行

## ステータス

採用

## コンテキスト

プロジェクトは当初、単一のRustクレートとしてAPIサーバーのみを提供していた。今後の拡張として以下が予定されている：

1. **CLIツール**: ターミナルからのインタラクティブなチャット機能
2. **Webフロントエンド**: ブラウザベースのチャットUI

これらは共通のビジネスロジック（OpenAI API呼び出し、セッション管理、データベース操作）を必要とする。

## 決定

フラットな単一クレート構成から、Cargo Workspaceを使用したモノレポ構成へ移行する。

```
rust-openai-api-wrapper/
├── Cargo.toml              # Workspace定義
├── ui/                     # フロントエンド（別エコシステム）
└── backend/
    ├── core/               # 共有ライブラリ
    │   └── src/db/migrations/  # sqlx migrations
    ├── api/                # Webサーバー (Axum)
    └── cli/                # CLIツール
```

### クレート責務

| クレート | 責務 | 依存関係 |
|---------|------|----------|
| `core` | ビジネスロジック、モデル、DB操作、OpenAI連携 | なし（Axum非依存） |
| `api` | HTTPハンドラー、エラーレスポンス変換 | `core` |
| `cli` | REPL、ローカルセッション管理 | `core` |

## 代替案

### 1. 単一クレート維持
- **メリット**: シンプル、設定不要
- **デメリット**: CLI追加時にAxum依存が含まれる、バイナリサイズ増大

### 2. 完全分離リポジトリ
- **メリット**: 独立したリリースサイクル
- **デメリット**: コード重複、依存管理の複雑化、開発効率低下

### 3. モノレポ（採用）
- **メリット**: コード共有、一括テスト、統一された依存バージョン
- **デメリット**: 初期設定の手間

## 結果

### 正の影響
- `core`クレートを通じてAPIとCLIでコードを共有
- Axum非依存の`core`により、CLIが軽量に保たれる
- `cargo test --workspace`で全クレートを一括テスト
- 依存バージョンがworkspace.dependenciesで統一管理

### 負の影響
- Cargo.tomlの設定が複雑化
- 新規参加者の学習コスト増

### 実装詳細

```toml
# Cargo.toml (workspace root)
[workspace]
resolver = "2"
members = [
    "backend/core",
    "backend/api",
    "backend/cli",
]

[workspace.dependencies]
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "uuid", "chrono"] }
# ... 共通依存
```

## 参考

- [Cargo Workspaces - The Rust Book](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
- [Monorepo vs Polyrepo](https://monorepo.tools/)
