// 나무 자르기

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .map(str::parse)
        .flatten();

    input.next();
    let m = input.next().unwrap();
    let trees = input.collect();

    println!("{}", solution(trees, m));
}

fn solution(trees: Vec<u64>, m: u64) -> u64 {
    let mut start = 0;
    let mut end = *trees.iter().max().unwrap();
    let check = |mid| {
        let mut sum = 0;
        for &tree in &trees {
            if mid <= tree {
                sum += tree - mid;
            }
        }

        sum >= m
    };

    while start + 1 < end {
        let mid = (start + end) / 2;
        if check(mid) {
            start = mid;
        } else {
            end = mid;
        }
    }

    start
}
