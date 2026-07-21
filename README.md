# competitive-programming

競技プログラミングの回答置き場。

## 構成

`atcoder/rust/` に 1 つの Cargo パッケージを置き、1問 = 1ファイルで `src/bin/` に並べる。

```
atcoder/rust/
├── Cargo.toml
└── src/bin/
    ├── abc300_a.rs
    └── abc300_b.rs
```

## セットアップ

```bash
cd atcoder/rust

cat > Cargo.toml <<'EOF'
[package]
name = "atcoder"
version = "0.0.0"
edition = "2021"

[dependencies]
proconio = { version = "0.5.0", features = ["derive"] }
EOF

mkdir -p src/bin

printf 'target/\nCargo.lock\n' > .gitignore
```

## 解き方

1. `atcoder/rust/src/bin/<コンテスト>_<問題>.rs` を作って解く
2. `cargo run --bin abc300_a` で実行し、サンプル入力を貼って動作確認（Ctrl+D で入力終了）
3. ファイルの中身を AtCoder の提出フォームに貼り付けて提出（言語は Rust (rustc 1.89.0)）

## 注意

- ジャッジ環境は Rust 1.89.0。使えるクレートとバージョンは
  [言語・ライブラリ一覧](https://img.atcoder.jp/file/language-update/2025-10/language-list.html) を参照
- 依存には proconio のみ入れてある。他のクレートが必要なら上記一覧にあるバージョンで追加する
