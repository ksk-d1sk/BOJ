// 숫자

use std::fmt::Write;

macro_rules! input_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap() ),+)
})}

fn main() {
    let mut output = String::new();
    let (a, b) = input_line!(u64, u64);
    let (l, s) = if a > b { (a, b) } else { (b, a) };
    let mut v = Vec::new();
    
    for i in s+1..l {
        v.push(i);
    }

    writeln!(output, "{}", v.len()).unwrap();
    for i in v {
        write!(output, "{} ", i).unwrap();
    }

    println!("{}", output);
}
