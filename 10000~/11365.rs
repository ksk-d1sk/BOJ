// !밀비 급일

use std::io::{self, Read, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut output = BufWriter::new(stdout);

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();

    for s in input.lines() {
        if s != "END" {
            for c in s.chars().rev() {
                write!(output, "{}", c).unwrap();
            }
            writeln!(output).unwrap();
        }
    }
}
