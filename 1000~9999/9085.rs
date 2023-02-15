// 더하기

use std::io::{self, Read, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut output = BufWriter::new(stdout);

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .map(str::parse::<usize>)
        .flatten();
    let mut next = || input.next().unwrap();

    for _ in 0..next() {
        let mut sum = 0;
        for _ in 0..next() { sum += next(); }
        writeln!(output, "{}", sum).unwrap();
    }
}
