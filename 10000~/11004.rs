// K번째 수

use std::io::{self, Read};

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .skip(1)
        .map(|e| e.parse().unwrap());

    let k = input.next().unwrap() as usize;
    let mut v: Vec<i32> = input.collect();

    let answer = v.select_nth_unstable(k - 1).1;

    println!("{}", answer);
}
