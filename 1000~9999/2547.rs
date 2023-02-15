// 사탕 선생 고창영

use std::io::{self, BufWriter, Write, Read};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut output = BufWriter::new(stdout.lock());

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .skip(1)
        .map(|e| e.parse::<u128>().unwrap());

    while let Some(n) = input.next() {
        let mut sum = 0;
        for _ in 0..n {
            sum += input.next().unwrap();
        }
        writeln!(output, "{}", if sum % n == 0 { "YES" } else { "NO" }).unwrap();
    }
}
