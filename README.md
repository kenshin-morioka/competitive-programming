# competitive-programming

競技プログラミングの回答置き場。

## 構成

サイトごとにディレクトリを分け、その配下に言語ごとのディレクトリを置く。

```
.
├── atcoder/
│   └── rust/
└── <他サイトを始めたら同様に追加 (leetcode/ など)>
```

各ディレクトリのセットアップ・解き方は以下のセクションに追記していく。

## atcoder

### rust

1 つの Cargo パッケージに、1問 = 1ファイルで `src/bin/` に並べる。

```
atcoder/rust/
├── Cargo.toml
└── src/bin/
    ├── abc300_a.rs
    └── abc300_b.rs
```

#### セットアップ

```bash
cd atcoder
cargo new rust --name atcoder                  # Cargo.toml / src / .gitignore を生成
cd rust
cargo add proconio@=0.5.0 --features derive    # 入力パース用クレートを追加
mkdir src/bin && rm src/main.rs                # 1問1ファイル運用のため src/bin に切り替え
```

#### 解き方

1. `src/bin/<コンテスト>_<問題>.rs` を作って解く
2. `cargo run --bin abc300_a` で実行し、サンプル入力を貼って動作確認（Ctrl+D で入力終了）
3. ファイルの中身を AtCoder の提出フォームに貼り付けて提出（言語は Rust (rustc 1.89.0)）

#### 注意

- ジャッジ環境は Rust 1.89.0。使えるクレートとバージョンは
  [言語・ライブラリ一覧](https://img.atcoder.jp/file/language-update/2025-10/language-list.html) を参照
- 依存には proconio のみ入れてある。他のクレートが必要なら上記一覧にあるバージョンで追加する
