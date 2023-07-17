// 공 넣기

use std::io::{self, Read};
use std::fmt::Write;

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .map(|e| e.parse().unwrap());
    let mut input = || input.next().unwrap();

    let mut v = vec![0_usize; input()]; 

    for _ in 0..input() {
        let mut i = input() - 1;
        let j = input();
        let k = input();
        while i < j {
            v[i] = k;
            i += 1;
        }
    }

    for i in v {
        write!(output, "{} ", i).unwrap();
    }

    print!("{output}");
}