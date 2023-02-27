// 약수 구하기

use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut input = buf.split_ascii_whitespace().map(|e| e.parse::<usize>().unwrap());

    let n = input.next().unwrap();
    let k = input.next().unwrap();
    let mut v = Vec::new();

    for i in 1..=(n / 2) {
        if n % i == 0 {
            v.push(i);
        }
    }

    v.push(n);

    print!("{}", v.get(k - 1).unwrap_or(&0));
}
