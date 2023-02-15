// 더하기

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let input = input.trim().split(',')
        .map(|e| e.parse::<u32>().unwrap());

    println!("{}", input.into_iter().sum::<u32>());
}
