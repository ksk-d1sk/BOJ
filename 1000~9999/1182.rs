// 부분수열의 합

use std::io::{self, Read};

struct BOJ1182 {
    s: i32,
    v: Vec<i32>,
    count: usize
}

impl BOJ1182 {
    fn new(s: i32, v: Vec<i32>) -> Self {
        Self { s, v, count: 0 }
    }

    fn solve(&mut self) -> usize {
        self.dfs(0, 0);

        if self.s == 0 {
            self.count -= 1;
        }

        self.count
    }

    fn dfs(&mut self, total: i32, depth: usize) {
        if depth < self.v.len() {
            self.dfs(total, depth + 1);
            self.dfs(total + self.v[depth], depth + 1);
        } else {
            if total == self.s {
                self.count += 1;
            }
        }
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .skip(1)
        .map(|e| e.parse().unwrap());

    let s = input.next().unwrap();
    let v: Vec<i32> = input.collect();

    let mut boj_1182 = BOJ1182::new(s, v);
    println!("{}", boj_1182.solve());
}