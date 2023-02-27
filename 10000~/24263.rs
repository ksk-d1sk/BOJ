// 알고리즘 수업 - 알고리즘의 수행 시간 2

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();
    println!("{}\n1", n);
}
