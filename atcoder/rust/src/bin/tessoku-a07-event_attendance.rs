// 自分の回答 (ACだが O(N * D))
// use proconio::input;
// use std::io::{self, BufWriter, Write};
//
// fn main() {
//     input! {
//         d:usize,
//         n:usize,
//         lr:[(usize,usize);n],
//     }
//
//     let mut ans = vec![0u64; d + 1];
//
//     let out = io::stdout();
//     let mut out = BufWriter::new(out.lock());
//
//     for (l, r) in lr {
//         for i in l..=r {
//             ans[i] += 1;
//         }
//     }
//     for i in 1..=d {
//         writeln!(out, "{}", ans[i]).unwrap();
//     }
// }

// 添削:
// - 計算量: 元の回答は区間ごとに毎日 +1 するので O(N * D)。
//   全員が全期間出席すると 10^5 * 10^5 = 10^10 回の加算になり本来は TLE 級。
// - imos法: 区間 [l, r] への +1 を「入退室の記録」に置き換える。
//   diff[l] += 1   → 日 l に入室 (+1人)
//   diff[r+1] -= 1 → 日 r+1 に退室 (日 r まではいるので抜けるのは翌日)
//   各区間の処理が O(1) になり、記録 O(N) + 累積 O(D) = 全体 O(N + D)。
// - 復元: diff を先頭から累積和すると「その日の在室人数」が順に求まる。
//   cur += diff[i] を i = 1..=d で回しながら cur を出力すればよい。
// - 配列サイズ: r = d のとき diff[d+1] に書き込むので、diff の長さは d + 2 が必要。
// - 型: 途中の diff の値は負になり得るので u64 ではなく i64 を使う。
//   (累積後の在室人数は必ず 0 以上になる)

// 添削後の回答
use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        d:usize,
        n:usize,
        lr:[(usize,usize);n],
    }

    let mut diff = vec![0i64; d + 2];
    for (l, r) in lr {
        diff[l] += 1;
        diff[r + 1] -= 1;
    }

    let out = io::stdout();
    let mut out = BufWriter::new(out.lock());

    let mut curr = 0i64;
    for i in 1..=d {
        curr += diff[i];
        writeln!(out, "{}", curr).unwrap();
    }
}
