// N과 M (3)

use std::io::{self, Read, BufWriter, Write, StdoutLock};

struct BOJ15651 {
    arr: Vec<usize>,
    n: usize,
    m: usize,
}

impl BOJ15651 {
    fn new(n: usize, m: usize) -> Self {
        let arr = Vec::with_capacity(m);
        BOJ15651 { arr, n, m }
    }

    fn solve(&mut self, output: &mut BufWriter<StdoutLock>) {
        if self.arr.len() == self.m {
            for elem in &self.arr {
                write!(output, "{} ", elem).unwrap();
            }
            writeln!(output).unwrap();
        } else {
            for i in 0..self.n {
                self.arr.push(i + 1);
                self.solve(output);
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

    let mut boj_15651 = BOJ15651::new(n, m);
    boj_15651.solve(&mut output);
}
