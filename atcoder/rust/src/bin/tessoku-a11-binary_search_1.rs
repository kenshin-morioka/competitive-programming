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
