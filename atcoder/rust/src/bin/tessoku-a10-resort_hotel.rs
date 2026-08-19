// 自分の回答 (O(N * D) で TLE 級)
// use proconio::input;
// use std::io::{self, BufWriter, Write};
//
// fn main() {
//     input! {
//         n:usize,
//         a:[u32;n],
//         d:usize,
//         lr:[(usize,usize);d],
//     }
//
//     let out = io::stdout();
//     let mut out = BufWriter::new(out.lock());
//
//     for (l, r) in lr {
//         let mut room = Vec::<u32>::new();
//         room.extend(&a[..l - 1]);
//         room.extend(&a[r - 1..]);
//
//         writeln!(out, "{}", room.iter().max().unwrap()).unwrap();
//     }
// }

// 添削:
// - 計算量: 各クエリで Vec を作り直して max を取ると 1 クエリ O(N)。
//   全体で O(N * D) = 最悪 10^5 * 10^5 = 10^10 回で TLE 級。
//   さらに毎クエリ Vec のメモリ確保 + 要素コピーが走るので定数倍も重い。
// - 考え方: クエリで聞かれるのは常に「前半 a[0..l-1] の max」と「後半 a[r..] の max」の
//   大きい方。つまり「先頭からの max」と「末尾からの max」を前計算しておけば
//   各クエリ O(1) で答えられる (累積和の max 版 = prefix max / suffix max)。
// - 前計算: pre[i] = a[0..i] の最大値、suf[i] = a[i..] の最大値。
//   答えは max(pre[l-1], suf[r])。前計算 O(N)、クエリ O(1) で全体 O(N + D)。
// - 境界: l = 1 のとき前半が空、r = n のとき後半が空になる。
//   pre / suf を長さ n+1 で作り pre[0] = 0, suf[n] = 0 の番兵を入れると分岐不要
//   (問題の制約で「全室休業」は来ないので、空側は 0 のままで max に影響しない)。
// - usize の引き算: 番兵なしで pre[l-2] のように書くと l = 1 で underflow panic する。
//   番兵方式なら添字は pre[l-1] / suf[r] だけで済み、引き算の事故が起きない。
// - 型: a_i ≤ 10^9 なので u32 で収まっている点は OK (10^9 < 2^32)。

// 添削後の回答
use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        n:usize,
        a:[u32;n],
        d:usize,
        lr:[(usize,usize);d],
    }

    let mut pre = vec![0u32; n + 1];
    let mut suf = vec![0u32; n + 1];

    for i in 0..n {
        pre[i + 1] = pre[i].max(a[i]);
    }

    for i in (0..n).rev() {
        suf[i] = suf[i + 1].max(a[i]);
    }

    let out = io::stdout();
    let mut out = BufWriter::new(out.lock());

    for (l, r) in lr {
        writeln!(out, "{}", pre[l - 1].max(suf[r])).unwrap();
    }
}

// memo1:
// x.max(y)は「xとyのmax」を返す
