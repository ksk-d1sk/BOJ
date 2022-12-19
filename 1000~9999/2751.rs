// 수 정렬하기 2

use std::io::{self, Read};
use std::fmt::Write;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().map(|i| i.parse::<i32>().unwrap());

    let mut output = String::new();
    let n = input.next().unwrap();
    let mut v = Vec::new();

    for _ in 0..n {
        v.push(input.next().unwrap());
    }

    v.sort_unstable();

    for i in v {
        writeln!(output, "{}", i).unwrap();
    }

    print!("{output}");
}
