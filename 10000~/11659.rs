// 구간 합 구하기 4

use std::io::{self, Read};
use std::fmt::Write;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    macro_rules! get_line {
        ( $($t:ty),+ ) => {{
            let mut iter = input.next().unwrap().split_ascii_whitespace();
            ( $(iter.next().unwrap().parse::<$t>().unwrap()),+ )
        }};
        () => { input.next().unwrap().split_ascii_whitespace().flat_map(str::parse::<u32>).collect::<Vec<_>>() };
    }

    let mut output = String::new();
    let (n, m) = get_line!(usize, usize);
    let mut arr = get_line!();

    for i in 0..n {
        arr[i] = if i != 0 { arr[i - 1] } else { 0 } + arr[i];
    }

    for _ in 0..m {
        let (i, j) = get_line!(usize, usize);
        writeln!(output, "{}", arr[j - 1] -if i != 1 { arr[i - 2] } else { 0 }).unwrap();
    }

    print!("{output}");
}