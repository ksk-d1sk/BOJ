// 중복 빼고 정렬하기

use std::io::{self, Read, BufWriter, Write};
use std::collections::BTreeSet;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut output = BufWriter::new(stdout);

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();

    input.split_ascii_whitespace()
        .skip(1)
        .map(str::parse)
        .flatten()
        .collect::<BTreeSet<i16>>()
        .into_iter()
        .for_each(|e| write!(output, "{} ", e).unwrap());
}
