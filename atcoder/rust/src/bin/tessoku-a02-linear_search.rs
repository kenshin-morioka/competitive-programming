use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u32,
        a: [u32; n],
    }
    println!("{}", if a.contains(&x) { "Yes" } else { "No" });
}
