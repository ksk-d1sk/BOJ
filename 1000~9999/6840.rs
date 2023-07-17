// Who is in the middle?

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut v: Vec<u8> = input
        .split_ascii_whitespace()
        .map(|e| e.parse().unwrap())
        .collect();

    println!("{}", v.select_nth_unstable(1).1);
}