// 自分の回答 (AC, O(N + H*W) — 想定解の二次元imos法)
use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        h:usize,
        w:usize,
        n:usize,
        queries:[(usize,usize,usize,usize);n],
    }

    let mut diff = vec![vec![0isize; w + 2]; h + 2];
    for (a, b, c, d) in queries {
        diff[a][b] += 1;
        diff[a][d + 1] -= 1;
        diff[c + 1][b] -= 1;
        diff[c + 1][d + 1] += 1;
    }

    for i in 1..=h {
        for j in 1..w + 1 {
            diff[i][j] += diff[i][j - 1];
        }
    }
    for i in 1..h + 1 {
        for j in 1..=w {
            diff[i][j] += diff[i - 1][j];
        }
    }

    let out = io::stdout();
    let mut out = BufWriter::new(out.lock());

    for i in 1..=h {
        let line = (1..=w)
            .map(|j| diff[i][j].to_string())
            .collect::<Vec<_>>()
            .join(" ");
        writeln!(out, "{}", line).unwrap();
    }
}

// memo1:
// 標準出力で末尾の改行をさせたくない場合はlnなしのマクロを使う

// memo2:
// collect: イテレータの要素を集めてコレクション(Vec, String, HashMap 等)に変換する。
//   変換先は型注釈かターボフィッシュで指定する (例: .collect::<Vec<_>>())。
// join: スライスの要素を区切り文字で連結して1つの String にする (例: vec.join(" "))。
//   数値はそのまま join できないので、先に .map(|x| x.to_string()) で文字列化してから
//   collect → join する。空白区切りの1行出力の定番パターン。

// 添削:
// - アルゴリズム: 二次元imos法で正解。ナイーブに矩形を毎回塗ると O(N * H * W) = 最悪 4*10^9 で TLE。
//   「4隅への記録 + 横方向累積和 + 縦方向累積和」に分解して O(N + H*W) にするのが想定解で、
//   4点の符号 (+1, -1, -1, +1) も正しい。1-indexed のまま扱い、c+1 / d+1 用に +2 の余白を
//   取っているのも良い(境界チェック不要になる)。
// - 累積和を in-place で行うテクニックも良い。diff 配列をそのまま上書きすれば追加メモリ不要。
// - 範囲記法の不統一: `1..=h` と `1..h + 1`、`1..=w` と `1..w + 1` が混在している。
//   同じ意味なら `1..=h` / `1..=w` に統一する。読み手が「違いがあるのか?」と疑ってしまう。
// - 命名: 累積和を取った後の diff はもう「差分」ではなく「積雪量そのもの」。
//   変数名は diff より snow / grid 等の方が後半の意味に合う。もしくは累積和適用を
//   関数に切るなら diff のままでも良い。
// - 型: isize はポインタ幅依存の型なので、競プロでは値域から i32 / i64 を選ぶのが慣習
//   (ここでは最大 N = 1000 なので i32 で十分。迷ったら i64)。
// - 出力: map → collect → join は定番で OK。BufWriter を使っているのも正しい
//   (H*W = 4*10^6 行分の出力があり得るので、都度 println! だと遅い)。
