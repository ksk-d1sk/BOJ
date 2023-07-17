// 문자와 문자열

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let mut s = input.next().unwrap().chars();
    let i: usize = input.next().unwrap().parse().unwrap();

    println!("{}", s.nth(i - 1).unwrap());
}