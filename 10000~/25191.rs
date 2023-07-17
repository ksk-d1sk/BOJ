// 치킨댄스를 추는 곰곰이를 본 임스

use std::io::{self, Read};

fn main() {
    let mut input  = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .map(|e| e.parse::<u16>().unwrap());

    let chicken = input.next().unwrap();
    let coke = input.next().unwrap();
    let beer = input.next().unwrap();

    println!("{}", chicken.min(coke / 2 + beer));
}