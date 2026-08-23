use proconio::input;
use std::{
    io::{self, BufWriter, Write},
    iter::TakeWhile,
};

fn main() {
    input! {
        n:usize,
        k:u32,
        a:[u32;n],
    }

    let out = io::stdout();
    let mut out = BufWriter::new(out.lock());

    let mut count = 0;
    let mut seconds = 0;

    for i in 0..10.poe(9).enumerate() {
        while count < k {}
    }
}
