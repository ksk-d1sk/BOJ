// 나이순 정렬

use std::io::{self, Read};
use std::fmt::Write;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let mut output = String::new();
    let n: u32 = input.next().unwrap().parse().unwrap();
    let mut v = Vec::new();

    for i in 0..n {
        v.push((i, input.next().unwrap().parse::<u8>().unwrap(), input.next().unwrap()));
    }

    v.sort_unstable_by(|a, b|
        if a.1 == b.1 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    );

    for (_, age, name) in v.iter() {
        writeln!(output, "{} {}", age, name).unwrap();
    }

    print!("{output}");
}
