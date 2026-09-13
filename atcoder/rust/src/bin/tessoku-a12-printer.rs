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
