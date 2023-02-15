// 대충 더해

use std::io::{self, BufRead, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut output = BufWriter::new(stdout);

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let (left, right) = (input.next().unwrap(), input.next().unwrap());
    let max;
    let min;

    if left.len() > right.len() {
        max = left.as_bytes();
        min = right.as_bytes();
    } else {
        max = right.as_bytes();
        min = left.as_bytes();
    }

    for i in 0..max.len() {
        if max.len() - i <= min.len() {
            write!(
                output,
                "{}",
                max[i] + min[(min.len() as i8 - max.len() as i8 + i as i8) as usize] - 96
            ).unwrap();
        } else {
            write!(output, "{}", max[i] - 48).unwrap();
        }
    }
}
