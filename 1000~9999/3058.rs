// 짝수를 찾아라

use std::io::{self, BufWriter, Read, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut output = BufWriter::new(stdout.lock());

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut iter = input.split_ascii_whitespace()
      .map(str::parse)
      .flatten();
    let mut next = || iter.next().unwrap();

    for _ in 0..next() {
      let mut v = Vec::new();
      for _ in 0..7 { v.push(next()); }
      let (sum, min) = solution(v);
      writeln!(output, "{} {}", sum, min).unwrap();
    }
}

#[inline]
fn solution(v: Vec<usize>) -> (usize, usize) {
  let iter = v.iter()
    .filter(|e| **e & 1 == 0);

  (iter.clone().sum(), *iter.min().unwrap())
}
