// 곱셈

macro_rules! input_line {
    ($($t: ty), +) => ({
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_ascii_whitespace();
        ($(iter.next().unwrap().parse::<$t>().unwrap()), +)
    });
}

fn main() {
    let (a, b, c) = input_line!(u64, u64, u64);
    println!("{}", solution(a, b, c));
}

fn solution(c: u64, n: u64, m: u64) -> u64 {
    let mut answer = 1;

    if n == 1 {
        answer *= c % m;
    } else {
        let temp = solution(c, n / 2, m);
        answer = (temp * temp) % m;
        if n & 1 == 1 {
            answer = (answer * c) % m;
        }
    }

    answer
}
