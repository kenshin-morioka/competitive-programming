use proconio::input;

fn main() {
    input! {
        n:u32,
        k:u32,
    }
    let mut ans = 0;

    for r in 1..=n {
        for b in 1..=n {
            if r + b >= k {
                break;
            };
            if k - r - b <= n {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
