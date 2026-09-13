use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        h:usize,
        w:usize,
        x:[[usize;w];h],
        q:usize,
        queries:[(usize,usize,usize,usize);q],
    }

    let mut s = vec![vec![0usize; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            s[i + 1][j + 1] = s[i + 1][j] + s[i][j + 1] - s[i][j] + x[i][j];
        }
    }

    let out = io::stdout();
    let mut out = BufWriter::new(out.lock());
    for (a, b, c, d) in queries {
        let ans = s[c][d] - s[a - 1][d] - s[c][b - 1] + s[a - 1][b - 1];
        writeln!(out, "{}", ans).unwrap();
    }
}
