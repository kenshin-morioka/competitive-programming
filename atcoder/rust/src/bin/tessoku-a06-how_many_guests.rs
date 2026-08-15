// 自分の回答 (TLE)
// use proconio::input;
//
// fn main() {
//     input! {
//         n:usize,
//         q:usize,
//         a:[u32;n],
//         lr:[(usize, usize);q],
//     }
//
//     for i in lr {
//         let tmp = &a[(i.0 - 1)..i.1];
//         let ans: u32 = tmp.iter().sum();
//         println!("{}", ans);
//     }
// }
//
// memo1:
// a[2]       // i32   → サイズが分かる → OK
// a[0..2]    // [i32] → サイズが分からない → そのまま値として扱えない
// &a[0..2]   // &[i32] → 参照自体のサイズは分かる → OK
// memo2:
// Rust のスライス範囲 start..end は 「start は含む、end は含まない」

// 添削:
// - 計算量: 元の回答はクエリごとに区間を舐めて合計するので O(N * Q)。
//   N = Q = 10^5 だと 10^10 回になり TLE。
//   累積和を前計算しておけば 1 クエリ O(1) で答えられ、全体 O(N + Q) になる。
// - 累積和のイディオム: 長さ n+1 の配列を作り s[0] = 0 としておくと、
//   l = 1 のときも s[l-1] = s[0] = 0 で成立し、場合分けが不要になる。
// - 添字: 入力は 1-indexed、Rust は 0-indexed だが、
//   s[r] は a[0..r] の和なので s[r] - s[l-1] がちょうど a の l 番目〜r 番目の和になる。
// - オーバーフロー: A_i <= 10^9、N <= 10^5 で合計は最大 10^14。u32 (最大約 4.3*10^9) では溢れるので u64 を使う。
// - 出力: println! は呼び出すたびに必ず flush されるわけではない。
//   ただし、大量の出力では標準出力への書き込み処理のオーバーヘッドが積み重なる。
//   BufWriter を使うと出力をバッファに溜めてまとめて書き出せるため、効率がよい。

use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        n:usize,
        q:usize,
        a:[u64;n],
        lr:[(usize, usize);q],
    }

    let mut s = vec![0u64; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }

    let out = io::stdout();
    let mut out = BufWriter::new(out.lock());
    for (l, r) in lr {
        writeln!(out, "{}", s[r] - s[l - 1]).unwrap();
    }
}

// memo3:
// Vec::new()は要素数0の空のVecを作成するが、vec!なら最初から要素が入った配列を作成できる
// memo4:
// io::stdout()は標準出力を取得。普段println!が出している先を変数として持つ。
// memo5:
// out.lock()は標準出力をロック。Rustのstdout()は複数のスレッドから使える仕組みなので、
// そのまま何度も書き込むとロック処理が発生するため。
// BufWriter→stdoutへの実際の書き込み回数を減らす
// lock()→その実際の書き込み時にロックを取り直さなくて済む
// memo6:
// BufWriter::new(...)は「この出力先に直接毎回書かず、一旦バッファに溜めるWriterを作る」
// memo7:
// writeln!はprintln!の出力先指定版。第一引数で指定して、それ以降は同じ。
// memo8:
// .unwrap()はwriteln!が書き込みに成功したか失敗したかをResultで返すため。
// println!の戻り値が()なのに対してwriteln!はResultなので、Result<(), std::io::Error>
// つまり、出力に成功したか失敗したかという結果を返す。
// これを無視すると、unused `Result` that must be used と警告が出る
// unwrap()とすることで、OK(())→そのまま続行。Err(...)→panicして終了としている。
// 競プロでは出力失敗を細かく処理する必要がほぼないので「失敗したらpanic」でいいとしている。
