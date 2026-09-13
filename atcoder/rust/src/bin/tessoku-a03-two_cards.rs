// for x in vec     : vecの所有権を消費する。ループ後はvecを使えない。
// for x in &vec    : vecを借用する。xは&Tになる。
// for &x in &vec   : vecを借用しつつ、xはTとして受け取る（T: Copyの場合）。
// qは外側のループで何度も使うので、&qとして借用する。

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u32,
        p: [u32;n],
        q: [u32;n],
    }

    for i in p {
        for &j in &q {
            if i + j == k {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
