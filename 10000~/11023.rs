// 더하기 3

use std::io::{self, Read, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut output = BufWriter::new(stdout);

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();

    let answer = input
        .split_ascii_whitespace()
        .map(str::parse::<u32>)
        .flatten()
        .sum::<u32>();

    writeln!(output, "{}", answer).unwrap();
}
