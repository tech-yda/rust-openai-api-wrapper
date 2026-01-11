# ADR-004: UI技術選定

## ステータス

採用

## コンテキスト

本プロジェクトは個人用ChatGPTクライアントとしてスタートし、将来的にはチーム内ツールへグロースさせる予定。Webフロントエンドを追加するにあたり、技術スタックを選定する必要がある。

### 要件
- ChatGPT風のチャットUI
- セッション管理（一覧表示、切替、削除）
- マークダウンレンダリング
- システムプロンプトのカスタマイズ
- 将来的なSSE対応（ストリーミングレスポンス）

### 選定基準
1. 学習目的（新しい技術を試したい）
2. 開発体験（TypeScript、ホットリロード）
3. Rustバックエンドとの親和性
4. コンポーネントライブラリの充実度

## 決定

**TanStack Start + shadcn/ui + Tailwind CSS** を採用する。

| カテゴリ | 技術 | 備考 |
|---------|------|------|
| フレームワーク | TanStack Start | ベータ版、学習目的で許容 |
| UIコンポーネント | shadcn/ui | コピー&ペースト方式、カスタマイズ性高 |
| スタイリング | Tailwind CSS | ユーティリティファースト |
| パッケージマネージャー | pnpm | 高速、ディスク効率 |

## 代替案

### 1. Next.js (App Router)
- **メリット**: 安定性、豊富なエコシステム、Vercelとの統合
- **デメリット**: 学習済み、新規性に欠ける、React Server Componentsの複雑さ

### 2. SvelteKit
- **メリット**: シンプル、軽量、コンパイラベース
- **デメリット**: エコシステムがReactより小さい、shadcn/ui非対応

### 3. Leptos (Rust)
- **メリット**: フルスタックRust、型安全
- **デメリット**: エコシステム未成熟、UIライブラリ不足、学習コスト高

### 4. TanStack Start（採用）
- **メリット**: 最新のReactパターン、TanStack Routerとの統合、学習機会
- **デメリット**: ベータ版、破壊的変更のリスク

## 結果

### 正の影響
- TanStack Router/Queryのファーストクラスサポート
- shadcn/uiによる高品質なUIコンポーネント
- Tailwind CSSによる迅速なスタイリング
- 最新のReactパターンを学習できる

### 負の影響
- ベータ版のため、破壊的変更の可能性
- 本番利用には追加検証が必要
- ドキュメントが発展途上

### リスク軽減策
- 個人プロジェクトのため、破壊的変更は許容
- 重要なビジネスロジックはバックエンド（Rust）に集中
- UIは比較的薄く保ち、必要に応じて別フレームワークへ移行可能

## 実装詳細

```bash
# プロジェクト作成
cd ui
pnpm create @tanstack/start

# shadcn/ui セットアップ
pnpm dlx shadcn-ui@latest init
pnpm dlx shadcn-ui@latest add button card input textarea scroll-area dialog
```

### ディレクトリ構成
```
ui/
├── src/
│   ├── routes/           # ファイルベースルーティング
│   ├── components/       # UIコンポーネント
│   │   ├── ui/          # shadcn/uiコンポーネント
│   │   ├── ChatMessage.tsx
│   │   ├── ChatInput.tsx
│   │   └── SessionList.tsx
│   └── lib/
│       └── api.ts        # APIクライアント
├── package.json
└── Dockerfile.dev
```

## 参考

- [TanStack Start](https://tanstack.com/start)
- [shadcn/ui](https://ui.shadcn.com/)
- [Tailwind CSS](https://tailwindcss.com/)
