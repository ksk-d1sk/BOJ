// 사장님 도박은 재미로 하셔야 합니다

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!(
        "{}",
        input.split_ascii_whitespace()
            .rev()
            .skip(1)
            .map(str::parse::<u32>)
            .flatten()
            .sum::<u32>()
    );
}