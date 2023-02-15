// AC

use std::io::{self, Read, BufWriter, Write};
use std::collections::VecDeque;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut output = BufWriter::new(stdout.lock());

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! input {
        () => { input.next().unwrap() };
        ( $t:ty ) => { input.next().unwrap().parse::<$t>().unwrap() };
    }

    for _ in 0..input!(u8) {
        let p = input!();
        let n = input!(usize);
        let v = input!();

        if n < p.chars().filter(|e| *e == 'D').count() {
            writeln!(output, "error").unwrap();
            continue;
        }

        let mut check = true;
        let mut v: VecDeque<u32> = v[1..(v.len() - 1)]
            .split(',')
            .map(str::parse)
            .flatten()
            .collect();

        let mut exe = |check| {
            if check {
                v.pop_front()
            } else {
                v.pop_back()
            }
        };

        for c in p.chars() {
            if c == 'R' {
                check ^= true;
            } else {
                exe(check);
            }
        }

        write!(output, "[").unwrap();

        if let Some(x) = exe(check) {
            write!(output, "{}", x).unwrap();
        }

        while let Some(x) = exe(check) {
            write!(output, ",{}", x).unwrap();
        }

        writeln!(output, "]").unwrap();
    }
}
