// 영수증

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .map(|e| e.parse::<u32>().unwrap());

    println!("{}", input.next().unwrap() - input.sum::<u32>());
}