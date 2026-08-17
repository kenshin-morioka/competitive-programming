// 自分の回答 (O(Q * H * W))
// use proconio::input;
// use std::io::{self, BufWriter, Write};
//
// fn main() {
//     input! {
//         h:usize,
//         w:usize,
//         x:[[usize;w];h],
//         q:usize,
//         a:[[usize;4];q],
//     }
//
//     let out = io::stdout();
//     let mut out = BufWriter::new(out.lock());
//     for j in a {
//         let mut s = 0usize;
//         for (i, v) in x.iter().enumerate() {
//             if i + 1 < j[0] || i + 1 > j[2] {
//                 continue;
//             }
//             s += v[j[1] - 1..j[3]].iter().sum::<usize>();
//         }
//         writeln!(out, "{}", s).unwrap();
//     }
// }

// memo1:
// マス目のような入力を受け取る時はVec<Vec<>>のような多次元配列で受け取る。
// マスの情報が文字列の場合：
// ```
// use proconio::{input, maker::Chars};
// ...
// input! {
//     h:useize,
//     w:usize,
//     x:[Chars;h],
// }
// ```
// とする。wの入力時の制約は競プロではかけないことが多い。
// マスの情報が整数で空白区切りの場合：回答の通りに書く

// 添削:
// - 計算量: 元の回答はクエリごとに矩形内を全走査するので O(Q * H * W)。
//   H, W <= 1500, Q <= 10^5 なので最悪 2.25 * 10^11 回で TLE 級。
// - 考え方: A06 の 1 次元累積和の 2 次元版。「毎回足し直す」のではなく
//   「前計算した累積和の引き算」でクエリを O(1) にする。
// - 2次元累積和: s[i][j] = 左上 (1,1) から (i,j) までの矩形の総和。
//   s[i][j] = s[i-1][j] + s[i][j-1] - s[i-1][j-1] + x[i][j]
//   (上と左を足すと左上が二重に足されるので 1 回引く: 包除原理)
// - 復元: 矩形 (a,b)-(c,d) の総和は
//   s[c][d] - s[a-1][d] - s[c][b-1] + s[a-1][b-1]
//   (大きい矩形から上と左をはがし、二重に引いた左上を足し戻す)
// - 実装の注意: s を (H+1) x (W+1) で確保し 0 行目・0 列目を 0 にしておくと
//   a-1, b-1 の境界処理が if 分岐なしで書ける。全体 O(H * W + Q)。
// - クエリの受け取り: a:[[usize;4];q] より [(usize, usize, usize, usize); q] と
//   タプルで受けると、for (a, b, c, d) in queries と名前付きで分解できて
//   j[0], j[1] より読みやすい。
// - usize の引き算: 復元式は s[c][d] - s[a-1][d] - s[c][b-1] + s[a-1][b-1] の
//   順序なら途中経過が常に非負でアンダーフローしない(引く順を変えると負になり得る)。
//   不安なら i64 で持つ流儀もある。
// - オーバーフロー: X <= 100, マス数 <= 1500 * 1500 なので総和は最大 2.25 * 10^8。
//   64bit の usize なら余裕。値が大きい問題では u64/i64 を意識する。

// 添削後の回答
use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        h:usize,
        w:usize,
        x:[[usize;w];h],
        q:usize,
        queries:[(usize,usize,usize,usize);q],
    }

    let mut s = vec![vec![0usize; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            s[i + 1][j + 1] = s[i + 1][j] + s[i][j + 1] - s[i][j] + x[i][j];
        }
    }

    let out = io::stdout();
    let mut out = BufWriter::new(out.lock());
    for (a, b, c, d) in queries {
        let ans = s[c][d] - s[a - 1][d] - s[c][b - 1] + s[a - 1][b - 1];
        writeln!(out, "{}", ans).unwrap();
    }
}
