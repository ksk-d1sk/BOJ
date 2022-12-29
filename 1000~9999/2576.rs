// 홀수

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.split_ascii_whitespace().map(|e| e.parse::<u16>().unwrap());

    let mut v = Vec::new();

    for x in input {
        if x & 1 == 1 {
            v.push(x);
        }
    }

    v.sort_unstable();

    if v.is_empty() {
        println!("-1");
    } else {
        println!("{}\n{}", v.iter().sum::<u16>(), v[0]);
    }
}
