// 수 정렬하기 4

use std::io::{self, Read, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut output = BufWriter::new(stdout.lock());

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();

    let mut v: Vec<i32> = input
        .split_ascii_whitespace()
        .skip(1)
        .map(str::parse)
        .flatten()
        .collect();

    v.sort_unstable_by(|a, b| b.cmp(a));

    for elem in v.iter() {
        writeln!(output, "{}", elem).unwrap();
    }
}
