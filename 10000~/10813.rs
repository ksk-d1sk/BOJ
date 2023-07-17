// 공 바꾸기

use std::io::{self, Read};
use std::fmt::Write;

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! input {
        ( $( $t:ty ), + ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = input!(u8, u8);
    let mut v: Vec<u8> = (1..=n).collect();

    for _ in 0..m {
        let (i, j) = input!(usize, usize);
        (v[i - 1], v[j - 1]) = (v[j - 1], v[i - 1]);
    }

    for elem in v {
        write!(output, "{} ", elem).unwrap();
    }

    print!("{output}");
}