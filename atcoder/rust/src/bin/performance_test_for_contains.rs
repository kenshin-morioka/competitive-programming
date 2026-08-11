// 組み込み関数の実装と制御構文のみの実装のパフォーマンス比較
// RUSTFLAGS="-C target-cpu=native" cargo run --release --bin {ファイル名}
// で実行することで、ホストOSのCPU専用にコード生成する指定
// （他のCPUで動かなくなる可能性があるので配布用バイナリ作成時は指定しない）

use std::hint::black_box;
use std::time::Instant;

fn by_for(a: &[u32], x: u32) -> bool {
    for v in a {
        if *v == x {
            return true;
        }
    }
    false
}

fn by_contains(a: &[u32], x: u32) -> bool {
    a.contains(&x)
}

fn main() {
    let n = 10_000_000;
    let a: Vec<u32> = (0..n).collect();

    // 最悪ケース: 最後に見つかる
    let x = n - 1;

    let repeat = 100;

    let start = Instant::now();
    for _ in 0..repeat {
        black_box(by_for(black_box(&a), black_box(x)));
    }
    println!("for      : {:?}", start.elapsed());

    let start = Instant::now();
    for _ in 0..repeat {
        black_box(by_contains(black_box(&a), black_box(x)));
    }
    println!("contains : {:?}", start.elapsed());
}
