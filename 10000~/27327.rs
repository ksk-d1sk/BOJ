// 時間 (Hour)

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: u16 = input.trim().parse().unwrap();

    println!("{}", n * 24);
}
