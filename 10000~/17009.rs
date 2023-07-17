// Winning Score

use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|e| e.parse::<usize>().unwrap());

    let mut input = || input.next().unwrap();

    let apple = input() * 3 + input() * 2 + input();
    let banana = input() * 3 + input() * 2 + input();

    println!(
        "{}",
        if apple == banana { 'T' }
        else if apple > banana { 'A' }
        else { 'B' }
    )
}