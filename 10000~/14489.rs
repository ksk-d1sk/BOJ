// 치킨 두 마리 (...)

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .map(|e| e.parse::<u32>().unwrap());
    let mut next = || input.next().unwrap();

    let sum = next() + next();
    let chicken_price = next() * 2;

    println!(
        "{}",
        if sum < chicken_price {
            sum
        } else {
            sum - chicken_price
        }
    )
}