// 재귀의 귀재

use std::io::{self, Read};
use std::fmt::Write;

struct Solution(usize);

impl Solution {
    fn new() -> Self {
        Solution(0)
    }

    fn solve(&mut self, s: &[u8], i: usize, j: usize) -> usize {
        self.0 += 1;
        if i >= j { 1 }
        else if s[i] != s[j] { 0 }
        else {
            self.solve(s, i + 1, j - 1)
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut output = String::new();

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();

    for s in input.split_ascii_whitespace().skip(1).map(|e| e.as_bytes()) {
        let mut boj_25501 = Solution::new();
        writeln!(
            output,
            "{} {}",
            boj_25501.solve(s, 0, s.len() - 1),
            boj_25501.0
        ).unwrap();
    }

    print!("{output}");
}
