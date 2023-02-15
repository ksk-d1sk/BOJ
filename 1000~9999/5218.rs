// 알파벳 거리

use std::io::{self, BufWriter, Read, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut output = BufWriter::new(stdout.lock());

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut iter = input.split_ascii_whitespace();

    macro_rules! next {
        () => {
            iter.next().unwrap()
        };
        ( $t:ty ) => {
            iter.next().unwrap().parse::<$t>().unwrap()
        };
    }

    for _ in 0..next!(u8) {
        let left = next!();
        let right = next!();

        write!(output, "Distances: ").unwrap();
        for i in solution(left, right) {
            write!(output, "{} ", i).unwrap();
        }
        writeln!(output).unwrap();
    }
}

fn solution(left: &str, right: &str) -> Vec<u8> {
    let mut answer = Vec::new();
    let left = left.as_bytes();
    let right = right.as_bytes();

    for i in 0..left.len() {
        let a = left[i];
        let mut b = right[i];

        if b < a {
            b += 26;
        }

        answer.push(b - a);
    }

    answer
}
