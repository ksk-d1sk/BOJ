// 알고리즘 수업 - 알고리즘의 수행 시간 3

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u64 = input.trim().parse().unwrap();

    println!("{}\n2", n * n);
}