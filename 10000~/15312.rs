// 이름 궁합

use std::io::{self, Read};

const STROKE: [u8; 26] = [
//  A  B  C  D  E  F  G  H  I  J  K  L  M  N  O  P  Q  R  S  T  U  V  W  X  Y  Z
    3, 2, 1, 2, 3, 3, 2, 3, 3, 2, 2, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1, 1, 2, 2, 1
];

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let you = input.next().unwrap();
    let her = input.next().unwrap();

    let result = solution(you, her);
    println!("{:02}", result);
}

fn solution(you: &str, her: &str) -> u8 {
    assert_eq!(you.len(), her.len());

    let you = you.as_bytes();
    let her = her.as_bytes();
    let mut v = Vec::new();

    for i in 0..you.len() {
        v.push(STROKE[(you[i] - b'A') as usize]);
        v.push(STROKE[(her[i] - b'A') as usize]);
    }

    for i in 0..v.len() - 2 {
        for j in 0..v.len() - i - 1 {
            v[j] = (v[j] + v[j + 1]) % 10;
        }
    }

    v[0] * 10 + v[1]
}
