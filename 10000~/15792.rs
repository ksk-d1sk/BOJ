// A/B - 2

use std::fmt::Write;

macro_rules! input_line { ($($t: ty), +) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap() ),+)
})}

fn main() {
    let (mut a, b) = input_line!(u32, u32);
    let mut output = String::new();
    write!(output, "{}.", a / b).unwrap();
    a %= b;

    for _ in 0..1000 {
        a *= 10;
        write!(output, "{}", a / b).unwrap();
        a %= b;
        if a == 0 { break; }
    }

    println!("{}", output);
}
