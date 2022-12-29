// 랜선 자르기

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .map(str::parse)
        .flatten();

    input.next();
    let n = input.next().unwrap();
    let v = input.collect();

    println!("{}", solution(n, v));
}

fn solution(n: u32, v: Vec<u32>) -> u32 {
    let mut start = 1;
    let mut end = *v.iter().max().unwrap() + 1;

    let check = |mid| {
        let mut sum = 0;
        for i in &v {
            sum += i / mid;
        }

        sum >= n
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
