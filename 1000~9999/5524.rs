// 입실 관리

use std::io::{self, Read};
use std::fmt::Write;

fn main() {
    let mut output = String::new();
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input.lines()
        .skip(1)
        .for_each(|e| writeln!(output, "{}", e.to_ascii_lowercase()).unwrap());

    print!("{output}");
}
