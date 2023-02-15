// ATM

use std::io::{self, Read};

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();

    let mut sumsum = 0;
    let mut sum = 0;
    let mut p: Vec<u32> = input
        .split_ascii_whitespace()
        .skip(1)
        .map(str::parse)
        .flatten()
        .collect();

    p.sort_unstable();

    for i in p.iter() {
        sum = i + sum;
        sumsum += sum;
    }

    println!("{}", sumsum);
}
