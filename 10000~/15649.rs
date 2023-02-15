// N과 M (1)

use std::io::{self, Read, BufWriter, Write, StdoutLock};

struct BOJ15649 {
    arr: Vec<usize>,
    is_used: Vec<bool>,
    n: usize,
    m: usize,
}

impl BOJ15649 {
    fn new(n: usize, m: usize) -> Self {
        let arr = Vec::new();
        let is_used = vec![false; n];
        BOJ15649 { arr, is_used, n, m }
    }

    fn solve(&mut self, output: &mut BufWriter<StdoutLock>) {
        if self.arr.len() == self.m {
            for elem in &self.arr {
                write!(output, "{} ", elem).unwrap();
            }
            writeln!(output).unwrap();
        } else {
            for i in 0..self.n {
                if !self.is_used[i] {
                    self.arr.push(i + 1);
                    self.is_used[i] = true;
                    self.solve(output);
                    self.arr.pop();
                    self.is_used[i] = false;
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let stdout = io::stdout();
    let mut output = BufWriter::new(stdout.lock());

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! next {
        ( $t:ty ) => { input.next().unwrap().parse::<$t>().unwrap() };
    }

    let (n, m) = (next!(usize), next!(usize));

    let mut boj_15649 = BOJ15649::new(n, m);
    boj_15649.solve(&mut output);
}
