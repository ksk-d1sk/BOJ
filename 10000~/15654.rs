// N과 M (5)

use std::io::{self, Read, BufWriter, Write, StdoutLock};

struct BOJ15654 {
    buf: Vec<u16>,
    is_used: Vec<bool>,
    arr: Vec<u16>,
    n: usize,
    m: usize,
}

impl BOJ15654 {
    fn new(arr: Vec<u16>,n: usize, m: usize) -> Self {
        let buf = Vec::new();
        let is_used = vec![false; n];
        BOJ15654 {buf, is_used, arr, n, m }
    }

    fn solve(&mut self, output: &mut BufWriter<StdoutLock>) {
        if self.buf.len() == self.m {
            for elem in &self.buf {
                write!(output, "{} ", elem).unwrap();
            }
            writeln!(output).unwrap();
        } else {
            for i in 0..self.n {
                if !self.is_used[i] {
                    self.buf.push(self.arr[i]);
                    self.is_used[i] = true;
                    self.solve(output);
                    self.buf.pop();
                    self.is_used[i] = false;
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut output = BufWriter::new(stdout.lock());

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! input {
        ( $t:ty ) => { input.next().unwrap().parse::<$t>().unwrap() };
        () => { input.map(|e| e.parse().unwrap()).collect() }
    }

    let (n, m) = (input!(usize), input!(usize));
    let mut arr: Vec<u16> = input!();

    arr.sort_unstable();

    let mut boj_15654 = BOJ15654::new(arr, n, m);
    boj_15654.solve(&mut output);
}
