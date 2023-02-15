// 절댓값 힙

use std::io::{self, Read, BufWriter, Write};
use std::collections::BinaryHeap;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut output = BufWriter::new(stdout.lock());

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let input = input
        .split_ascii_whitespace()
        .skip(1)
        .map(|e| e.parse::<i32>().unwrap());

    let mut heap = BinaryHeap::new();

    for i in input {
        if i != 0 {
            if 0 < i {
                heap.push((-i, false)); 
            } else {
                heap.push((i, true));
            }
        } else {
            let x = heap.pop().unwrap_or((0, true));
            writeln!(
                output,
                "{}",
                if x.1 { x.0 } else { -x.0 }
            ).unwrap();
        }
    }
}
