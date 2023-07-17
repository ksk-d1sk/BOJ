// 1, 2, 3 더하기

use std::io::{self, Read};
use std::fmt::Write;

const MAX: usize = 12;

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(str::parse::<usize>);

    let mut arr = [0; MAX];
    arr[0] = 1;
    arr[1] = 2;
    arr[2] = 4;

    for i in 0..(MAX - 3) {
        arr[i + 3] = arr[i] + arr[i + 1] + arr[i + 2];
    }

    for n in input {
        writeln!(output, "{}", arr[n - 1]).unwrap();
    }

    print!("{output}");
}