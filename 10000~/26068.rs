// 치킨댄스를 추는 곰곰이를 본 임스 2

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!(
        "{}",
        input.split_ascii_whitespace()
            .skip(1)
            .map(|e| e[2..].parse::<u16>().unwrap())
            .filter(|e| *e <= 90)
            .count()
    );
}