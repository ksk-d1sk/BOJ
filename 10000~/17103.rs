// 골드바흐 파티션

use std::io::{self, Read};
use std::fmt::Write;

const MAX: usize = 1_000_000;

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(str::parse::<usize>);

    let prime_list = get_prime_list(MAX);

    for n in input {
        let mut count = 0;
        let mut s = 2;
        let mut e = n - 2;
        while s <= e {
            if prime_list[s] && prime_list[e] {
                count += 1;
            }
            s += 1;
            e -= 1;
        }
        writeln!(output, "{count}").unwrap();
    }

    print!("{output}");
}

fn get_prime_list(m: usize) -> Vec<bool> {
    let mut prime_list = vec![true; m + 1];

    for i in 2..=(m as f64).sqrt() as usize {
        if prime_list[i] {
            for j in ((i * i)..=m).step_by(i) {
                prime_list[j] = false;
            }
        }
    }

    prime_list
}