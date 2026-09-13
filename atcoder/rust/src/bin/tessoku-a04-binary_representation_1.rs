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
