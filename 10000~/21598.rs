// SciComLove

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

    for _ in 0..input.next().unwrap() {
        writeln!(output, "SciComLove").unwrap();
    }
}
