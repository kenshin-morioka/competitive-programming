// 自分の回答
// use proconio::input;
//
// fn main() {
//     input! {
//         n: i32,
//         x: i32,
//         a: [i32;n],
//     }
//     for i in a {
//         if i == x {
//             println!("Yes");
//             return;
//         }
//     }
//     println!("No")
// }

// 添削後
use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u32,
        a: [u32; n],
    }
    println!("{}", if a.contains(&x) { "Yes" } else { "No" });
}
