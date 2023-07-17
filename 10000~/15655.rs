// N과 M (6)

use std::io::{self, BufWriter, Read, Write, StdoutLock};

struct BOJ15655 {
    n: usize,
    m: usize,
    arr: Vec<u16>,
    output_arr: Vec<u16>,
}

impl BOJ15655 {
    fn new(n: usize, m: usize, arr: Vec<u16>) -> Self {
        let output_arr = Vec::new();
        Self { n, m, arr, output_arr }
    }

    fn solve(&mut self, output: &mut BufWriter<StdoutLock>, depth: usize) {
        if self.output_arr.len() >= self.m {
            for elem in &self.output_arr {
                write!(output, "{} ", elem).unwrap();
            }
            writeln!(output).unwrap();
        } else {
            for i in depth..self.n {
                self.output_arr.push(self.arr[i]);
                self.solve(output, i + 1);
                self.output_arr.pop();
            }
        }
    }
}

fn main() {
    let stdin  = io::stdin();
    let stdout = io::stdout();
    let mut stdin  = stdin.lock();
    let mut output = BufWriter::new(stdout.lock());

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! input {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()), +) };
        () => { input.map(|e| e.parse().unwrap()).collect() };
    }

    let (n, m) = input!(usize, usize);
    let mut arr: Vec<u16> = input!();

    arr.sort_unstable();

    let mut boj_15655 = BOJ15655::new(n, m, arr);
    boj_15655.solve(&mut output, 0);
}
