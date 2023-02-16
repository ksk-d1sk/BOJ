// N번째 큰 수

use std::io::{self, Read};
use std::fmt::Write;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut output = String::new();

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let input = input.lines().skip(1);

    for line in input {
        let mut v: Vec<u16> = line
            .split_ascii_whitespace()
            .map(|e| e.parse().unwrap())
            .collect();

        writeln!(output, "{}", v.select_nth_unstable(7).1).unwrap();
    }

    print!("{output}");
}
