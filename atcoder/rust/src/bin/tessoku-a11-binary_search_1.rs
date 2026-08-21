// 自分の回答 (TLE: x == a[mid] のとき無限ループ)
// use proconio::input;
// use std::io::{self, BufWriter, Write};
//
// fn main() {
//     input! {
//         n:usize,
//         x:u32,
//         mut a:[u32;n],
//     }
//
//     let out = io::stdout();
//     let mut out = BufWriter::new(out.lock());
//
//     // 全探索（線形探索）
//     // for (i, v) in a.iter().enumerate() {
//     //     if v == &x {
//     //         writeln!(out, "{}", i + 1).unwrap();
//     //     }
//     // }
//
//     let mut i: Vec<usize> = (1..=n).collect();
//     let mut mid: usize;
//     while a.len() != 1 {
//         mid = a.len() / 2;
//         if x < a[mid] {
//             a = a[..mid].to_vec();
//             i = i[..mid].to_vec();
//         } else if x > a[mid] {
//             a = a[mid + 1..].to_vec();
//             i = i[mid + 1..].to_vec();
//         }
//     }
//     writeln!(out, "{}", i[0]).unwrap();
// }

// memo1:
// to_vec() をしないと型の不一致のためエラーになる
// Vec<u32> と [u32]（または &[u32]）は別の型
// Vec<u32>は要素を所有している、サイズ変更可能なコンテナ
// [u32]は連続した要素の範囲を表す型
// memo2:
// 初期値が不要な変数を定義する場合は、let a;（可変ならlet mut a;）で良い。
// 型は最初に代入される値で決まるが、事前に指定したいならlet a: u32;

// 添削:
// - TLE の原因は計算量ではなくロジックのバグ。x == a[mid] のとき if も else if も
//   成立せず、a が一切縮まらないまま while が回り続けて無限ループになる。
//   二分探索には「一致したら答えを出して終了」の分岐が必須。3 分岐で考える:
//   一致 → mid が答え / a[mid] < x → 右半分へ / a[mid] > x → 左半分へ。
// - 部分配列を to_vec() でコピーし続ける必要はない。探索範囲を表す添え字 2 つ
//   (半開区間 [left, right) が定石) を動かすだけでよく、コピーはゼロになる。
//   こうすると添え字ベクタ i も不要 (答えはそのまま mid + 1) で、a も mut 不要。
// - mid は毎周ループ内で let mid = ... と定義すれば mut 変数を外に持たなくてよい
//   (memo2 の「初期値不要な変数」を作るより、そもそもループ内 let が自然)。
// - 出力が 1 行だけなら BufWriter は不要。println! で十分。
// - Rust イディオム: ソート済みスライスには標準の a.binary_search(&x) がある。
//   戻り値は Result<usize, usize> (見つかれば Ok(添え字)、なければ Err(挿入位置))
//   なので unwrap() で中身を取り出す。この問題は 1 行で解ける。
// - 追記: x > a[mid] のときの left = mid; は、x が必ず存在するこの問題の制約下
//   では実は正しく動く (範囲サイズ 2 以上なら mid >= left + 1 で必ず縮み、
//   サイズ 1 まで縮めばその要素が x)。ただし x が存在しない入力だと、サイズ 1 で
//   x > a[mid] のとき left = mid が空回りして無限ループする (例: a = [1, 3],
//   x = 2)。a[mid] != x は確定している (直前の == 分岐を通過済み) ので、
//   mid 自身を除外する left = mid + 1; が存在しない場合にも壊れない定石。

// 添削後の回答
use proconio::input;

fn main() {
    input! {
        n:usize,
        x:u32,
        a:[u32;n],
    }

    let mut left = 0;
    let mut right = n;
    while left < right {
        let mid = (left + right) / 2;

        if x == a[mid] {
            println!("{}", mid + 1);
            return;
        } else if x < a[mid] {
            right = mid;
        } else if x > a[mid] {
            left = mid + 1;
        }
    }
}
