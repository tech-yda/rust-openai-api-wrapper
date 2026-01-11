# ADR-0006: テスト戦略

## ステータス
採用

## 日付
2026-01-11

## コンテキスト
現状、テストコードが存在しない。APIの品質保証と回帰防止のため、テスト戦略を策定する必要がある。

## 決定

### テストレベル
統合テスト（APIエンドポイントテスト）のみを実装する。

| レベル | 採用 | 理由 |
|--------|------|------|
| ユニットテスト | 不採用 | ビジネスロジックが単純、統合テストで十分 |
| 統合テスト | 採用 | APIエンドポイントの動作を保証 |
| E2Eテスト | 不採用 | UIがないため不要 |

### ディレクトリ構成

```
src/
├── main.rs          # 最小限（サーバー起動のみ）
├── lib.rs           # pub fn app() -> Router（テスト用に公開）
├── handlers/
├── services/
└── ...

tests/
└── api_tests.rs     # 統合テスト
```

**設計ポイント**:
- `main.rs`はブートストラップのみ
- `lib.rs`に`app()`関数を公開してテストから使用
- axum公式推奨パターンに準拠

### テストツール

| ツール | バージョン | 用途 |
|--------|-----------|------|
| axum-test | 16.x | HTTPリクエストシミュレーション |
| tokio (test feature) | 1.x | 非同期テスト実行 |

**axum-testを選択した理由**:

| 手法 | 特徴 | 推奨度 |
|------|------|--------|
| **axum-test** | モックネットワーク、簡潔なAPI | ⭐ 採用 |
| tower::Service | 直接呼び出し、公式例で使用 | ○ |
| reqwest + spawn | 実サーバー、本番に近い | △ |

### テスト項目

| エンドポイント | メソッド | 期待結果 |
|---------------|---------|---------|
| /health | GET | 200 OK |
| /sessions | POST | 200 OK（セッションID返却） |
| /sessions/{id} | GET | 200 OK（存在時）、404（不存在時） |
| /sessions/{id}/chat | POST | 200 OK（レスポンス返却） |
| /sessions/{id} | DELETE | 204 No Content |

### データベース戦略

| 方式 | 採用 | 理由 |
|------|------|------|
| テスト用DB | 採用 | 本番に近い動作を確認 |
| トランザクションロールバック | 不採用 | 設定が複雑 |
| モック | 不採用 | 実装コストが高い |

**テスト実行フロー**:
1. docker-compose up -d db でテスト用DBを起動
2. 各テストで必要に応じてテーブルをクリア
3. cargo test で統合テストを実行

### テストコード例

```rust
use axum_test::TestServer;
use your_app::app;

#[tokio::test]
async fn test_create_session() {
    let server = TestServer::new(app()).unwrap();

    let response = server.post("/sessions")
        .json(&json!({"system_prompt": "You are helpful"}))
        .await;

    response.assert_status_ok();
    let body: serde_json::Value = response.json();
    assert!(body.get("id").is_some());
}

#[tokio::test]
async fn test_session_not_found() {
    let server = TestServer::new(app()).unwrap();

    let response = server.get("/sessions/00000000-0000-0000-0000-000000000000")
        .await;

    response.assert_status_not_found();
}
```

## 検討したが採用しなかったもの

| 選択肢 | 理由 |
|--------|------|
| ユニットテスト重視 | ロジックが単純で費用対効果が低い |
| reqwest + 実サーバー | テスト実行が遅く、ポート競合のリスク |
| モックDB | 実装コストが高く、本番との乖離リスク |
| テストコンテナ | 現時点ではシンプルなdocker-composeで十分 |

## 参考資料
- [axum-test crate](https://docs.rs/axum-test)
- [axum公式テスト例](https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs)
- [Shuttle Launchpad: Testing Axum](https://www.shuttle.dev/launchpad/issues/2023-17-11-issue-12-testing-axum-applications)
- [How to Test Axum APIs](https://www.ruststepbystep.com/how-to-test-axum-apis-unit-and-integration-testing-guide/)

## 結果
- axum-testによる簡潔なテストコード
- lib.rs分離によるテスタビリティ向上
- 実DBを使用した信頼性の高いテスト
- CI/CDでの自動実行が容易
