// 최댓값

use std::io::{self, Read, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut output = BufWriter::new(stdout);

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let input = input
        .lines()
        .map(|e| e
            .split_ascii_whitespace()
            .map(str::parse::<u8>)
            .flatten()
        );

    let mut max = 0;
    let mut ac = 0;
    let mut ar = 0;

    for (c, iter) in input.enumerate() {
        for (r, elem) in iter.enumerate() {
            if max < elem {
                max = elem;
                ac = c;
                ar = r;
            }
        }
    }

    writeln!(output, "{}\n{} {}", max, ac + 1, ar + 1).unwrap();
}
