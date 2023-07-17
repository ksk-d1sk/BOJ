// 초코바

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n: u16 = input.next().unwrap().parse().unwrap();
    let m: u16 = input.next().unwrap().parse().unwrap();

    println!(
        "{}",
        if n * 100 >= m { "Yes" } else { "No" }
    );
}