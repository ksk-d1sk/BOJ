// The Fastest Sorting Algorithm In The World

use std::io::{self, Read, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut output = BufWriter::new(stdout);

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();

    for i in 1..=input.lines().skip(1).count() {
        writeln!(output, "Case {}: Sorting... done!", i).unwrap();
    }
}
