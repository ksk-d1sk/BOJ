// 피시방 알바

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
        .skip(1)
        .map(str::parse::<usize>)
        .flatten();
    let mut next = || input.next().unwrap();

    let mut count = 0;
    let mut seat = vec![false; 100];

    for i in input {
        if seat[i - 1] {
            count += 1;
        } else {
            seat[i - 1] = true;
        }
    }

    writeln!(output, "{}", count).unwrap();
}
