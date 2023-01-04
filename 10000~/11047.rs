// 동전 0

use std::io::{self, Read};

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .skip(1)
        .map(str::parse)
        .flatten();

    let k = input.next().unwrap();
    let coins = input.collect();

    println!("{}", solution(k, coins));
}

fn solution(mut k: usize, coins: Vec<usize>) -> usize {
    let mut count = 0;

    while 0 < k {
        let max_coin = coins
            .iter()
            .filter(|coin| **coin <= k)
            .max()
            .unwrap();

        count += k / max_coin;
        k %= max_coin;
    }

    count
}
