// 햄버거 만들기

use std::io::{self, Read, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut output = BufWriter::new(stdout);

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! next {
        ( $t:ty ) => { input.next().unwrap().parse::<$t>().unwrap() };
    }

    let (a, b) = (next!(u8), next!(u8));
    writeln!(output, "{}", (a / 2).min(b)).unwrap();
}
