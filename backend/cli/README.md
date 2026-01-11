# CLI

ターミナルから OpenAI API と対話するための CLI ツール。

## インストール

```bash
cargo install --path backend/cli
```

## コマンド

```bash
cli chat              # インタラクティブチャット
cli ask "質問"        # ワンショット質問
cli sessions list     # セッション一覧
cli sessions delete   # セッション削除
```

## 使用例

### ワンショット質問

```bash
$ cli ask "What is Rust?"
Rust is a systems programming language...

$ cli ask --system "You are a pirate" "Hello"
Ahoy, matey!
```

### インタラクティブチャット

```bash
$ cli chat
Welcome to Chat CLI! Type /help for commands.
> Hello!
Assistant: Hi there! How can I help you today?

> /save my-chat
Session saved to ~/.config/chat-cli/sessions/my-chat.json

> /exit
Goodbye!
```

### セッション読み込み

```bash
$ cli chat --load my-chat
Loaded session: my-chat (2 messages)
> Continue our conversation
```

## REPL コマンド

| コマンド | 説明 |
|---------|------|
| `/save <name>` | セッション保存 |
| `/load <name>` | セッション読み込み |
| `/list` | セッション一覧 |
| `/clear` | セッションクリア |
| `/help` | ヘルプ表示 |
| `/exit` | 終了 |

## 設定ファイル

`~/.config/chat-cli/config.toml`:

```toml
[default]
system_prompt = "You are a helpful assistant."
model = "gpt-4o-mini"
```

## ファイル構成

```
~/.config/chat-cli/
├── config.toml          # 設定
├── sessions/            # セッションJSON
│   └── my-chat.json
└── history/             # 入力履歴
    └── my-chat.txt
```

## 環境変数

| 変数 | 説明 |
|------|------|
| `OPENAI_API_KEY` | OpenAI APIキー（必須） |
