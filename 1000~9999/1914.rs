// 하노이 탑

use std::io::{self, BufWriter, Write, StdoutLock};

fn main() {
    let stdout = io::stdout();
    let mut output = BufWriter::new(stdout.lock());
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse().unwrap();

    writeln!(output, "{}", (1_u128 << n) - 1).unwrap();

    if n <= 20 {
        solution(&mut output, n, 1, 3);
    }
}

fn solution(output: &mut BufWriter<StdoutLock>, n: u8, start: u8, end: u8) {
    if n == 1 {
        writeln!(output, "{} {}", start, end).unwrap();
    } else {
        solution(output, n - 1, start, 6 - start - end);
        writeln!(output, "{} {}", start, end).unwrap();
        solution(output, n - 1, 6 - start - end, end);
    }
}
