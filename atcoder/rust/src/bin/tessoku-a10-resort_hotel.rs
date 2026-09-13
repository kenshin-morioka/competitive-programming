use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        n:usize,
        a:[u32;n],
        d:usize,
        lr:[(usize,usize);d],
    }

    let mut pre = vec![0u32; n + 1];
    let mut suf = vec![0u32; n + 1];

    for i in 0..n {
        pre[i + 1] = pre[i].max(a[i]);
    }

    for i in (0..n).rev() {
        suf[i] = suf[i + 1].max(a[i]);
    }

    let out = io::stdout();
    let mut out = BufWriter::new(out.lock());

    for (l, r) in lr {
        writeln!(out, "{}", pre[l - 1].max(suf[r])).unwrap();
    }
}

// memo1:
// x.max(y)は「xとyのmax」を返す
