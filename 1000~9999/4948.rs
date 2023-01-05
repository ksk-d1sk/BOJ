// 베르트랑 공준

use std::io::{self, Read, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut output = BufWriter::new(stdout);

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let input = input
        .split_ascii_whitespace()
        .map(str::parse::<usize>)
        .flatten();

    let list = get_prime_list(123456 * 2 + 1);

    for i in input {
        if i != 0 {
            writeln!(output, "{}", solution(&list, i)).unwrap();
        }
    }
}

fn get_prime_list(n: usize) -> Vec<bool> {
    let mut list = vec![true; n + 1];
    let sqrt_n = (n as f64).sqrt() as usize;

    list[0] = false;
    list[1] = false;

    for i in 2..=sqrt_n {
        if list[i] {
            for j in i..=(n / i) {
                list[i * j] = false;
            }
        }
    }

    list
}

fn solution(list: &Vec<bool>, n: usize) -> usize {
    let mut count = 0;
    let start = n + 1;
    let end = 3.max(n * 2);

    for i in start..end {
        if list[i] {
            count += 1;
        }
    }

    count
}
