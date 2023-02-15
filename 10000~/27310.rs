// :chino_shock:

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().as_bytes();

    let mut answer = input.len() + 2;

    for i in input {
        if *i == b'_' {
            answer += 5;
        }
    }

    println!("{answer}");
}
