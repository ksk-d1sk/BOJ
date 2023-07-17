// Equality

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let a = input.next().unwrap().parse::<u8>().unwrap();
    input.next();
    let b = input.next().unwrap().parse::<u8>().unwrap();
    input.next();
    let c = input.next().unwrap().parse::<u8>().unwrap();

    println!(
        "{}",
        if a + b == c { "YES" }
        else { "NO" }
    );
}