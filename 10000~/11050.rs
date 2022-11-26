// 이항 계수 1

macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap() ),+)
})}

fn main() {
    let (n, k) = parse_line!(u64, u64);

    println!("{}", factorial(n) / (factorial(k) * factorial(n - k)));
}

fn factorial(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => n * factorial(n - 1)
    }
}
