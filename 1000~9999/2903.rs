// 중앙 이동 알고리즘

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();
    let mut temp = 2;

    for _ in 0..n {
        temp = temp * 2 - 1;
    }

    print!("{}", temp * temp);
}