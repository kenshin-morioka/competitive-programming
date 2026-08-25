use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        d:usize,
        n:usize,
        lr:[(usize,usize);n],
    }

    let out = io::stdout();
    let mut out = BufWriter::new(out.lock());

    let mut diff = vec![0i32; d + 2];
    for (l, r) in lr {
        diff[l] += 1;
        diff[r + 1] -= 1;
    }

    let mut curr = 0i32;
    for i in 1..=d {
        curr += diff[i];
        writeln!(out, "{}", curr).unwrap();
    }
}
