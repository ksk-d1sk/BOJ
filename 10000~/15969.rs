// 행복

use std::io::{self, Read, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut output = BufWriter::new(stdout);

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut v: Vec<u16> = input
        .split_ascii_whitespace()
        .skip(1)
        .map(str::parse)
        .flatten()
        .collect();

    v.sort_unstable();
    
    writeln!(output, "{}", v[v.len() - 1] - v[0]).unwrap();
}
