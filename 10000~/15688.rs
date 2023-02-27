// 수 정렬하기 5

use std::io::{self, Read};
use std::fmt::Write;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut output = String::new();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();
    let mut v: Vec<i32> = buf.split_ascii_whitespace()
        .skip(1)
        .map(|e| e.parse().unwrap())
        .collect();

    v.sort_unstable();

    for i in v.iter() {
        writeln!(output, "{}", i).unwrap();
    }

    print!("{output}");
}
