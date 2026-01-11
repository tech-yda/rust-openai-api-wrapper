# ADR-0001: プロジェクトアーキテクチャ

## ステータス
採用

## 日付
2026-01-11

## コンテキスト
Rust初心者がOpenAI APIラッパーのAPIサーバーを構築する。学習目的のプロジェクトであり、Rustの基本概念を習得しながら実用的なアプリケーションを開発したい。

### 検討したアーキテクチャ

| パターン | 特徴 | 採用判断 |
|----------|------|----------|
| シンプル階層構造 | handlers/services/models、Rails/Express風 | **採用** |
| Clean Architecture | domain/application/infrastructure/presentation | 見送り |
| Onion Architecture | core/application/infrastructure/api | 見送り |
| MVC (Loco.rs) | Rails風フルスタック | 見送り |

## 決定
**シンプルな階層構造**を採用する。

```
src/
├── main.rs          # エントリーポイント
├── config.rs        # 設定管理
├── handlers/        # HTTPハンドラー（コントローラー相当）
├── models/          # データ構造・型定義
└── services/        # ビジネスロジック
```

## 理由
1. **学習効率**: Rustの基礎概念（所有権、トレイト、async等）に集中できる
2. **直感的**: TypeScript/Python経験者にとって馴染みやすい構造
3. **段階的発展**: Phase 2以降で必要に応じてClean Architectureへ移行可能
4. **オーバーエンジニアリング回避**: 小規模プロジェクトに過剰な抽象化は不要

## 各層の責務

| 層 | 責務 | TypeScript相当 |
|----|------|---------------|
| handlers | HTTP リクエスト/レスポンス処理 | Express route handler |
| services | ビジネスロジック | class / ロジック関数 |
| models | データ構造・型定義 | interface / type |

## 結果
- Rust初心者でも理解しやすい構造でプロジェクトを開始できた
- 各層の責務が明確で、コードの見通しが良い
- Phase 2でDB層を追加する際も、構造を大きく変更せずに拡張可能
