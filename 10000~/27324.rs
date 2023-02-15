// ゾロ目 (Same Numbers)

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n = input.trim().as_bytes();

    println!(
        "{}",
        if n[0] == n[1] { 1 } else { 0 }
    );
}
