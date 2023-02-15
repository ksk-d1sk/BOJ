// 초6 수학

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
        let left  = next();
        let right = next();
        writeln!(
            output, "{} {}",
            left * right / gcd(left, right),
            gcd(left, right)
        ).unwrap();
    }
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a }
    else { gcd(b, a % b) }
}
