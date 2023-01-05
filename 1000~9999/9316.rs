// Hello Judge

use std::io::{self, Write, BufWriter, BufRead};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout);

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let n = input.trim_end().parse::<u16>().unwrap();

    for i in 1..=n {
        writeln!(stdout, "Hello World, Judge {}!", i).unwrap();
    }
}
