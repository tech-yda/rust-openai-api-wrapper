# ADR-0008: モノレポ構成への移行

## ステータス
採用

## 日付
2026-01-11

## コンテキスト
現在のフラットなプロジェクト構造を、UIとCLIを追加するためにモノレポ構成へ移行する必要がある。coreロジック（OpenAI API呼び出し、DB操作、設定管理）をAPIサーバーとCLIツールで共有したい。

### 検討した構成

| パターン | 特徴 | 採用判断 |
|----------|------|----------|
| 単一クレート維持 | 現状維持、featureフラグで機能切替 | 見送り |
| Cargo Workspace (モノレポ) | 複数クレートで共有、型安全 | **採用** |
| 完全分離リポジトリ | core/api/cliを別リポジトリ | 見送り |

## 決定
**Cargo Workspace**を使用したモノレポ構成を採用する。

```
rust-openai-api-wrapper/
├── Cargo.toml              # Workspace定義
├── migrations/             # sqlx migrations（ルート直下）
├── ui/                     # フロントエンド
└── backend/
    ├── core/               # 共有ライブラリクレート
    ├── api/                # Webサーバー（バイナリクレート）
    └── cli/                # CLIツール（バイナリクレート）
```

## 理由
1. **コード再利用**: coreロジックをAPIとCLIで完全共有
2. **型安全**: 同一ワークスペース内で型を共有、不整合を防止
3. **一括ビルド**: `cargo build --workspace`で全クレートをビルド
4. **依存管理**: `[workspace.dependencies]`で共通依存を一元管理
5. **段階的開発**: 既存コードを維持しながら移行可能

## クレート構成

| クレート | 種類 | 責務 |
|----------|------|------|
| core | ライブラリ | OpenAIサービス、DB操作、設定、共通エラー型 |
| api | バイナリ | Axum Webサーバー、HTTPハンドラー |
| cli | バイナリ | インタラクティブCLI、ローカルセッション管理 |

## migrationsの配置
- **ルート直下** (`./migrations/`)に配置
- sqlxのデフォルトパスを維持
- `cargo sqlx prepare --workspace`と相性が良い

## エラーハンドリングの分離
- `core::error` - 基本エラー型（thiserror、Axum非依存）
- `api::error` - Axum用`IntoResponse`実装

## 結果
- APIとCLIで同一のOpenAIサービス・DB操作コードを使用可能
- 将来的なモジュール追加（認証、通知など）も容易
- ビルド時間の最適化（変更クレートのみ再ビルド）
