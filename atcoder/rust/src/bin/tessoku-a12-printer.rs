// 自分の回答 (シミュレーション。TLE 級 + 等号のバグあり)
// use proconio::input;
//
// fn main() {
//     input! {
//         n:usize,
//         k:u32,
//         a:[u32;n],
//     }
//
//     let mut count = 0;
//     let mut second = 0;
//
//     while count <= k {
//         second += 1;
//         a.iter().for_each(|x| {
//             if second % x == 0 {
//                 count += 1;
//             }
//         });
//     }
//
//     println!("{}", second);
// }

// 添削:
// - 計算量: 1 秒ずつ進めるシミュレーションは O(答え × N)。答えは最大 10^9、N は最大 10^5 なので
//   最悪 10^14 回のループになり TLE どころか終わらない。
// - 等号のバグ: `while count <= k` だと count がちょうど k に達しても loop を継続してしまい、
//   答えより 1 大きい値を出す (例: N=1, A=[1], K=5 → 正解 5 なのに 6 を出力)。`count < k` が正しい。
//   もっとも今回は計算量の時点で書き直しが必要。
// - 考え方の転換: 「K 枚目が印刷される時刻はいつか」を直接求めるのは難しいが、
//   「t 秒後までに K 枚以上印刷されているか?」という判定問題なら簡単。
//   t 秒後までにプリンター i が印刷した枚数は floor(t / A_i) なので、
//   合計 sum(t / A_i) >= K かどうかを O(N) で判定できる。
// - 単調性 → 二分探索: この判定は t について単調 (t が増えるほど枚数は増える) なので、
//   「答え (条件を満たす最小の t)」を二分探索できる。left=1, right=10^9 (問題文で答え <= 10^9 が保証)。
//   全体 O(N log(10^9)) ≈ 10^5 × 30 で余裕。A11 の「値の二分探索」に対し、これは「答えの二分探索」。
// - 型: sum(t / A_i) は最大 N × t = 10^5 × 10^9 = 10^14 で u32 (約 4.3×10^9) を溢れる。
//   K・A・合計はすべて u64 で受けるのが安全。
// - イディオム: `for_each` の中で外の変数を書き換えるより、
//   `a.iter().map(|&x| t / x).sum::<u64>()` のように map + sum で合計を作るほうが Rust らしい。

// 添削後の回答
use proconio::input;

fn main() {
    input! {
        n:usize,
        k:u64,
        a:[u64;n],
    }

    let mut left = 1;
    let mut right = 1_000_000_000;

    while left < right {
        let mid = (left + right) / 2;

        let total: u64 = a.iter().map(|x| mid / x).sum();
        if total >= k {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    println!("{}", left);
}
