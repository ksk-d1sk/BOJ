// Gorani Command

use std::io::{self, Read};

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .map(|e| e.parse().unwrap());

    let (n, m) = (input.next().unwrap() - 1, input.next().unwrap() - 1);
    let mut answer = (n, m);
    let v: Vec<usize> = input.collect();

    for a in 0..n {
        if v[a] < v[a + 1] {
            answer.0 = a;
            break;
        }
    }

    for b in n..(n + m) {
        if v[b] < v[b + 1] {
            answer.1 = b - n;
            break;
        }
    }

    println!("{} {}", answer.0 + 1, answer.1 + 1);
}
