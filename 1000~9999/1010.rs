// 다리 놓기

use std::fmt::Write;

macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap() ),+)
})}

fn main() {
    let mut output = String::new();
    let t = parse_line!(usize);

    for _ in 0..t {
        let (n, m) = parse_line!(u128, u128);
        writeln!(output, "{}", solution(n, m)).unwrap();
    }

    print!("{}", output);
}

fn solution(n: u128, m: u128) -> u128 {
    (1..=m).product::<u128>() / ((1..=n).product::<u128>() * (1..=(m - n)).product::<u128>())
}
