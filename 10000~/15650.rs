// N과 M (2)

use std::io::{self, Read, BufWriter, Write, StdoutLock};

struct BOJ15650 {
    arr: Vec<usize>,
    n: usize,
    m: usize,
}

impl BOJ15650 {
    fn input(n: usize, m: usize) -> Self {
        let arr = Vec::new();
        BOJ15650 { arr, n, m }
    }

    fn solve(&mut self, output: &mut BufWriter<StdoutLock>, num: usize) {
        if self.arr.len() == self.m {
            for elem in &self.arr {
                write!(output, "{} ", elem).unwrap();
            }
            writeln!(output).unwrap();
        } else {
            for i in num..self.n {
                self.arr.push(i + 1);
                self.solve(output, i + 1);
                self.arr.pop();
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

    macro_rules! input {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = input!(usize, usize);

    let mut boj_15650 = BOJ15650::input(n, m);
    boj_15650.solve(&mut output, 0);
}
