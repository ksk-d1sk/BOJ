// 세로읽기

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
        .map(|e| e.chars().collect());

    let mut board: Vec<Vec<char>> = Vec::new();
    for _ in 0..5 {
        board.push(input.next().unwrap());
    }

    for i in 0..15 {
        for v in &board {
            if let Some(c) = v.get(i) {
                write!(output, "{}", c).unwrap();
            }
        }
    }
}
