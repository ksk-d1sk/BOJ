// 어려운 소인수분해

use std::io::{self, Read};
use std::fmt::Write;

fn get_min_factor_list(n: usize) -> Vec<usize> {
    let mut min_factor_list: Vec<usize> = (0..=n).collect();
    let sqrt_n = (n as f64).sqrt() as usize;

    for i in 2..=sqrt_n {
        if min_factor_list[i] == i {
            for j in ((i * i)..=n).step_by(i) {
                if min_factor_list[j] == j {
                    min_factor_list[j] = i;
                }
            }
        }
    }

    min_factor_list
}

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(str::parse::<usize>);

    let min_factor_list = get_min_factor_list(5_000_000);

    for k in input {
        let mut n = k;

        while 1 < n {
            write!(output, "{} ", min_factor_list[n]).unwrap();
            n /= min_factor_list[n];
        }

        output.push('\n');
    }

    print!("{output}");
}