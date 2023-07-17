// 바구니 뒤집기

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
        reverse(&mut v, i, j);
    }

    for elem in v {
        write!(output, "{} ", elem).unwrap();
    }

    print!("{output}");
}

fn reverse(v: &mut Vec<u8>, i: usize, j: usize) {
    if i < j {
        (v[i - 1], v[j - 1]) = (v[j - 1], v[i - 1]);
        reverse(v, i + 1, j - 1);
    }
}