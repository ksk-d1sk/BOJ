// 카드 구매하기

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().map(|i| i.parse::<u32>().unwrap());

    let n = input.next().unwrap();
    let mut p = Vec::new();

    for _ in 0..n {
        p.push(input.next().unwrap());
    }

    let result = solution(p);
    println!("{result}");
}

fn solution(mut p: Vec<u32>) -> u32 {
    for i in 0..(p.len() / 2) {
        for j in i..(p.len() - i - 1) {
            p[i + j + 1] = (p[i] + p[j]).max(p[i + j + 1]);
        }
    }

    p[p.len() - 1]
}
