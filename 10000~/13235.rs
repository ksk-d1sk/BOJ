// 팰린드롬

use std::io;

struct Solution(usize);

impl Solution {
    fn new() -> Self {
        Solution(0)
    }

    fn solve(&mut self, s: &[u8], i: usize, j: usize) -> bool {
        self.0 += 1;
        if i >= j { true }
        else if s[i] != s[j] { false }
        else { self.solve(s, i + 1, j - 1) }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let s = input.trim().as_bytes();

    let mut boj_13235 = Solution::new();

    println!("{}", boj_13235.solve(s, 0, s.len() - 1));
}
