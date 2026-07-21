# competitive-programming

競技プログラミングの回答置き場。

## 構成

```
.
├── Makefile              # `make login` ターゲットを提供 (現状はAtCoderログイン用のみ)
├── scripts/
│   └── set_atcoder_cookie.sh
└── atcoder/
    └── rust/              # AtCoder を Rust で解く用 (cargo-compete)
        ├── compete.toml
        └── template-cargo-lock.toml
```

## セットアップ

### cargo-compete のインストール

```bash
cargo install cargo-compete
```

### AtCoder へのログイン

AtCoder のログインは reCAPTCHA が必須になっており、`cargo compete login atcoder` による
ユーザー名/パスワードの自動ログインは通らない（何度も入力を求められてループする）。

そのため、ブラウザで手動ログインしたセッションを `cargo-compete` に読み込ませる。

1. Chrome で `https://atcoder.jp` にログインする
2. DevTools (`Cmd+Option+I`) → Application → Cookies → `https://atcoder.jp` → `REVEL_SESSION` の Value をコピー
3. リポジトリルートで以下を実行し、コピーした値を貼り付ける

   ```bash
   make login
   ```

4. ログイン確認

   ```bash
   cd atcoder/rust
   cargo compete login atcoder
   # => "Already Logged in" と表示されればOK
   ```

セッションには有効期限があるため（AtCoder側の仕様に依存、目安半年程度）、
ログインが切れたら同じ手順 (`make login`) をやり直す。

## 使い方 (atcoder/rust)

```bash
cd atcoder/rust

# コンテストのテストケース取得 + プロジェクト作成
cargo compete new abc300

# 問題を解く
cd abc300
# src/bin/a.rs などを編集

# 手元でテスト
cargo compete test a

# 提出
cargo compete submit a
```

## リポジトリ運用

- 作業は必ずブランチを切って行い、PR経由で `main` にマージする
- PR マージ後、作業ブランチは自動削除される
- PR 作成時、作成者が自動でアサインされる
