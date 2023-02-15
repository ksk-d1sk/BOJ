// 2 番目に大きい整数 (The Second Largest Integer)

use std::io::{self, Read, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut output = BufWriter::new(stdout);

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut v: Vec<i32> = input
        .split_ascii_whitespace()
        .map(str::parse)
        .flatten()
        .collect();

    v.sort_unstable();

    writeln!(output, "{}", v[1]).unwrap();
}
