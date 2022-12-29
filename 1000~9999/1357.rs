// 뒤집힌 덧셈

macro_rules! input_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap() ),+)
})}

fn main() {
    let (a, b) = input_line!(String, String);
    println!("{}", solution(a, b));
}

#[inline]
fn solution(a: String, b: String) -> u16 {
    reverse_parse((reverse_parse(a) + reverse_parse(b)).to_string())
}

#[inline]
fn reverse_parse(s: String) -> u16 {
    s.chars().rev().collect::<String>().parse().unwrap()
}
