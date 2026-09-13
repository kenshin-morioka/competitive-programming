use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        h:usize,
        w:usize,
        n:usize,
        queries:[(usize,usize,usize,usize);n],
    }

    let mut diff = vec![vec![0i64; w + 2]; h + 2];
    for (a, b, c, d) in queries {
        diff[a][b] += 1;
        diff[a][d + 1] -= 1;
        diff[c + 1][b] -= 1;
        diff[c + 1][d + 1] += 1;
    }

    for i in 1..=h {
        for j in 1..=w {
            diff[i][j] += diff[i][j - 1];
        }
    }
    for i in 1..=h {
        for j in 1..=w {
            diff[i][j] += diff[i - 1][j];
        }
    }

    let out = io::stdout();
    let mut out = BufWriter::new(out.lock());

    for i in 1..=h {
        let line = (1..=w)
            .map(|j| diff[i][j].to_string())
            .collect::<Vec<_>>()
            .join(" ");
        writeln!(out, "{}", line).unwrap();
    }
}

// memo1:
// 標準出力で末尾の改行をさせたくない場合はlnなしのマクロを使う

// memo2:
// collect: イテレータの要素を集めてコレクション(Vec, String, HashMap 等)に変換する。
//   変換先は型注釈かターボフィッシュで指定する (例: .collect::<Vec<_>>())。
// join: スライスの要素を区切り文字で連結して1つの String にする (例: vec.join(" "))。
//   数値はそのまま join できないので、先に .map(|x| x.to_string()) で文字列化してから
//   collect → join する。空白区切りの1行出力の定番パターン。
