// Copier

use std::io::{self, BufWriter, Read, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut output = BufWriter::new(stdout.lock());

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let input = input
        .split_ascii_whitespace()
        .skip(1);
    
    for i in input {
        writeln!(output, "{} {}", i, i).unwrap()
    }
}
