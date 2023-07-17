// 문자열 집합

use std::io::{self, Read};
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! input {
        () => { input.next().unwrap() };
        ( $( $t:ty ),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()), +) };
    }

    let (n, m) = input!(usize, usize);
    let mut set = HashSet::with_capacity(n);
    let mut count = 0;

    for _ in 0..n {
        set.insert(input!());
    }

    for _ in 0..m {
        if set.contains(input!()) {
            count += 1;
        }
    }

    print!("{count}");
}