# ADR-0007: システムプロンプト戦略

## ステータス
採用

## 日付
2026-01-11

## コンテキスト
セッションベースのチャット機能において、システムプロンプト（AIの振る舞いを指定するプロンプト）をどのように管理するか決定する必要がある。

### 現状
- 単発チャット（`/chat`）ではリクエストごとに`system_prompt`を指定可能
- セッションチャット（`/sessions/{id}/chat`）ではシステムプロンプト未対応

## 決定

### 方式A: セッション作成時のみ設定（採用）

セッション作成時にシステムプロンプトを設定し、そのセッション内のすべてのチャットで使用する。

```
POST /sessions
{
  "system_prompt": "You are a helpful assistant. Always respond in Japanese."
}
```

### 検討した選択肢

| 方式 | メリット | デメリット | 採用 |
|------|---------|-----------|------|
| A: セッション作成時のみ | シンプル、一貫性 | 途中変更不可 | ⭐ 採用 |
| B: 各チャットで毎回指定 | 柔軟 | 毎回指定が面倒、履歴と矛盾する可能性 | 不採用 |
| C: 両方対応（デフォルト+上書き） | 最も柔軟 | 実装複雑、挙動が分かりにくい | 不採用 |

### 方式Aを選択した理由

1. **一貫性**: セッション全体で同じ振る舞いを保証
2. **シンプルさ**: 実装・理解が容易
3. **実用性**: 多くのチャットアプリ（ChatGPT等）と同じパターン
4. **学習目的**: Rustの基本パターンを学ぶのに十分

### 実装詳細

#### データモデル

```sql
-- sessions テーブルにカラム追加
ALTER TABLE sessions ADD COLUMN system_prompt TEXT;
```

```rust
pub struct Session {
    pub id: Uuid,
    pub system_prompt: Option<String>,  // 追加
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct CreateSessionRequest {
    pub system_prompt: Option<String>,
}
```

#### チャット時の動作

1. セッションからシステムプロンプトを取得
2. システムプロンプトがあれば、メッセージ履歴の先頭に追加
3. OpenAI APIに送信

```rust
// session_chat ハンドラーの処理
let mut messages = Vec::new();

// システムプロンプトがあれば先頭に追加
if let Some(system_prompt) = &session.system_prompt {
    messages.push(Message {
        role: "system".to_string(),
        content: system_prompt.clone(),
    });
}

// 履歴を追加
for msg in history {
    messages.push(Message {
        role: msg.role.clone(),
        content: msg.content.clone(),
    });
}

// 新しいユーザーメッセージを追加
messages.push(Message {
    role: "user".to_string(),
    content: request.message.clone(),
});
```

### API仕様

#### セッション作成

```bash
# システムプロンプト付き
curl -X POST http://localhost:3000/sessions \
  -H "Content-Type: application/json" \
  -d '{"system_prompt": "You are a helpful assistant."}'

# システムプロンプトなし（従来通り）
curl -X POST http://localhost:3000/sessions
```

#### レスポンス

```json
{
  "id": "uuid-here",
  "system_prompt": "You are a helpful assistant.",
  "created_at": "2026-01-11T12:00:00Z"
}
```

## 結果
- セッション単位でAIの振る舞いを設定可能
- シンプルで予測可能な動作
- 既存のセッションAPIとの互換性を維持（system_promptはオプション）
