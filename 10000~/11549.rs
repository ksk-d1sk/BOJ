// Identifying tea

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let t = input.next().unwrap();

    println!("{}", input.filter(|e| *e == t).count());
}