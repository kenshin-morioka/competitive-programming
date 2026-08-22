// 自分の回答
// use proconio::input;
// fn main() {
//     input! { mut n:u32 }
//     let mut arr: Vec<u32> = Vec::new();
//     // if n == 1 {
//     //     println!("{}", n);
//     //     return;
//     // }
//     // while n > 1 {
//     //     arr.insert(0, n % 2);
//     //     n /= 2;
//     // }
//     // arr.insert(0, n);
//     for _ in 0..10 {
//         arr.insert(0, n % 2);
//         n /= 2;
//     }
//     let result: String = arr.iter().map(|x| x.to_string()).collect();
//     println!("{}", result);
// }

// ===== 添削メモ =====
//
// 結論: 正解 (13→0000001101 / 1023→1111111111 / 1→0000000001 を確認済み)
//
// 1. Vec::insert(0, ...) は先頭挿入のたびに全要素を後ろへずらす O(要素数) の操作。
//    今回は10要素なので実害ないが、桁数が大きい問題だと O(桁数^2) になる。
//    定石は「push して最後に reverse」:
//      for _ in 0..10 {
//          arr.push(n % 2);
//          n /= 2;
//      }
//      arr.reverse();
//
// 2. Rust ならフォーマット指定子で1行で書ける。
//    {:010b} = 2進数・10桁・ゼロ埋め:
//      println!("{:010b}", n);
//    (この場合 n を mut にする必要もなくなる)
//
// 3. コメントアウトされた試行錯誤のコードはコミット前に削除する (履歴は git に残る)
//
// 4. 細かい点: map(|x| x.to_string()) は桁ごとに String を確保する。
//    数値→文字なら char 変換の方が軽い:
//      arr.iter().map(|x| char::from(b'0' + *x as u8)).collect()
//    ただしこの規模では好みの範囲。

// 添削後
use proconio::input;

fn main() {
    input! { mut n:u32 };
    let mut arr: Vec<u32> = Vec::new();

    for _ in 0..10 {
        arr.push(n % 2);
        n /= 2;
    }
    arr.reverse();

    let result: String = arr.iter().map(|x| x.to_string()).collect();
    println!("{}", result);
}

// memo1:
// .rev() → イテレータの順番を逆にする
// .reverse() → Vec やスライスそのものの要素順を逆にする
