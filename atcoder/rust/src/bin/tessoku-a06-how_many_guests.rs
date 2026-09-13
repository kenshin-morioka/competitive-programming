use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        n:usize,
        q:usize,
        a:[u64;n],
        lr:[(usize, usize);q],
    }

    let mut s = vec![0u64; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }

    let out = io::stdout();
    let mut out = BufWriter::new(out.lock());
    for (l, r) in lr {
        writeln!(out, "{}", s[r] - s[l - 1]).unwrap();
    }
}

// memo3:
// Vec::new()は要素数0の空のVecを作成するが、vec!なら最初から要素が入った配列を作成できる
// memo4:
// io::stdout()は標準出力を取得。普段println!が出している先を変数として持つ。
// memo5:
// out.lock()は標準出力をロック。Rustのstdout()は複数のスレッドから使える仕組みなので、
// そのまま何度も書き込むとロック処理が発生するため。
// BufWriter→stdoutへの実際の書き込み回数を減らす
// lock()→その実際の書き込み時にロックを取り直さなくて済む
// memo6:
// BufWriter::new(...)は「この出力先に直接毎回書かず、一旦バッファに溜めるWriterを作る」
// memo7:
// writeln!はprintln!の出力先指定版。第一引数で指定して、それ以降は同じ。
// memo8:
// .unwrap()はwriteln!が書き込みに成功したか失敗したかをResultで返すため。
// println!の戻り値が()なのに対してwriteln!はResultなので、Result<(), std::io::Error>
// つまり、出力に成功したか失敗したかという結果を返す。
// これを無視すると、unused `Result` that must be used と警告が出る
// unwrap()とすることで、OK(())→そのまま続行。Err(...)→panicして終了としている。
// 競プロでは出力失敗を細かく処理する必要がほぼないので「失敗したらpanic」でいいとしている。
