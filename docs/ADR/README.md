# Architecture Decision Records (ADR)

このディレクトリには、プロジェクトの設計決定を記録したADRが含まれています。

## ADR一覧

| # | タイトル | ステータス |
|---|----------|-----------|
| [0001](./0001-project-architecture.md) | プロジェクトアーキテクチャ | 採用 |
| [0002](./0002-technology-stack.md) | 技術スタック選定 | 採用 |
| [0003](./0003-openai-model-selection.md) | OpenAIモデル選定 | 採用 |
| [0004](./0004-phase2-database-plan.md) | Phase 2 - データベース導入計画 | 計画中 |

## ADRとは

Architecture Decision Records (ADR) は、ソフトウェアアーキテクチャに関する重要な決定を記録するためのドキュメントです。

各ADRには以下が含まれます：
- **コンテキスト**: 決定が必要になった背景
- **決定**: 採用した選択肢
- **理由**: なぜその選択をしたか
- **結果**: 決定による影響

## 新しいADRの作成

```bash
# ファイル名の形式: NNNN-kebab-case-title.md
touch docs/ADR/0005-new-decision.md
```

テンプレート:
```markdown
# ADR-NNNN: タイトル

## ステータス
提案中 / 採用 / 非推奨 / 却下

## 日付
YYYY-MM-DD

## コンテキスト
[決定が必要になった背景]

## 決定
[採用した選択肢]

## 理由
[なぜその選択をしたか]

## 結果
[決定による影響]
```
