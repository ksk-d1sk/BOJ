// 한다 안한다

use std::io::{self, Read};
use std::fmt::Write;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut output = String::new();

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();

    input.split_ascii_whitespace()
        .skip(1)
        .for_each(|e| {
            let arr = e.as_bytes();
            writeln!(
                output,
                "{}",
                if arr[arr.len() / 2 - 1] == arr[arr.len() / 2] {
                    "Do-it"
                } else {
                    "Do-it-Not"
                }
            ).unwrap()
        });

    print!("{output}");
}
