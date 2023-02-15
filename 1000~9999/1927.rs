// 최소 힙

use std::io::{self, Read, BufWriter, Write};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let stdout = io::stdout();
    let mut output = BufWriter::new(stdout.lock());

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let input = input
        .split_ascii_whitespace()
        .skip(1)
        .map(|e| e.parse::<u32>().unwrap());

    let mut heap = BinaryHeap::new();

    for i in input {
        if i == 0 {
            writeln!(output, "{}", heap.pop().unwrap_or(Reverse(0)).0).unwrap();
        } else {
            heap.push(Reverse(i));
        }
    }
}
