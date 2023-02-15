// 몇개고?

use std::io::{self, Read};

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .map(|e| e.parse::<u8>().unwrap());
    let mut next = || input.next().unwrap();

    let (t, s) = (next(), next());

    println!(
        "{}",
        if 12 <= t && t <= 16 && s == 0 {
            320
        } else {
            280
        }
    )
}
