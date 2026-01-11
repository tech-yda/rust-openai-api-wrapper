# ADR-0003: OpenAIモデル選定

## ステータス
採用

## 日付
2026-01-11

## コンテキスト
OpenAI APIのChat Completionsエンドポイントで使用するモデルを選定する必要がある。2026年1月時点で利用可能な最新モデルを調査した。

## 調査結果（2026年1月時点）

| モデルID | 説明 | 特徴 |
|----------|------|------|
| gpt-5.2 | GPT-5.2 Thinking | 推論モデル、複雑なタスク向け |
| **gpt-5.2-chat-latest** | GPT-5.2 Instant | ChatGPTで使用、バランス型 |
| gpt-5.2-pro | GPT-5.2 Pro | 最高精度、レイテンシ高い |

## 決定
**gpt-5.2-chat-latest** を採用する。

```rust
const MODEL: &str = "gpt-5.2-chat-latest";
```

## 理由
1. **ChatGPTと同等品質**: 実際のChatGPTで使用されているモデル
2. **バランス**: 精度とレイテンシのバランスが良い
3. **コスト効率**: Pro版より低コストで十分な品質
4. **知識カットオフ**: 2025年8月（最新）
5. **コンテキストウィンドウ**: 400K tokens

## モデル変更方法
`src/services/openai.rs` の定数を変更する：

```rust
// 現在の設定
const MODEL: &str = "gpt-5.2-chat-latest";

// 推論モデルに変更する場合
const MODEL: &str = "gpt-5.2";

// 最高精度モデルに変更する場合
const MODEL: &str = "gpt-5.2-pro";
```

## 将来の拡張
- モデルを環境変数で設定可能にする
- リクエストごとにモデルを指定可能にする

## 参考リンク
- [GPT-5.2 Model - OpenAI API](https://platform.openai.com/docs/models/gpt-5.2)
- [Introducing GPT-5.2 - OpenAI](https://openai.com/index/introducing-gpt-5-2/)
