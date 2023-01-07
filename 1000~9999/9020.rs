// 골드바흐의 추측

use std::io::{self, Read, BufWriter, Write};

struct Solution(Vec<bool>);

impl Solution {
    fn new(n: usize) -> Self {
        let mut list = vec![true; n + 1];
        let sqrt_n = (n as f64).sqrt() as usize;

        list[0] = false;
        list[1] = false;

        for i in 2..=sqrt_n {
            if list[i] {
                for j in i..=(n / i) {
                    list[i * j] = false;
                }
            }
        }

        Solution(list)
    }

    fn solution(&self, n: usize) -> (usize, usize) {
        assert_eq!(n & 1, 0);

        let mut answer = (0, 0);
        let mut i = 1;

        while i <= n - i {
            if self.0[i] && self.0[n - i] {
                answer = (i, n - i);
            }
            i += 1;
        }

        answer
    }
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut output = BufWriter::new(stdout);

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let input = input
        .split_ascii_whitespace()
        .skip(1)
        .map(str::parse)
        .flatten();

    let goldbach = Solution::new(10000);

    for i in input {
        if i != 0 {
            let (min, max) = goldbach.solution(i);
            writeln!(output, "{} {}", min, max).unwrap();
        }
    }
}
