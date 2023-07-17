// 골드바흐의 추측

use std::io::{self, Read};
use std::fmt::Write;

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input
        .split_ascii_whitespace()
        .map(|e| e.parse().unwrap());

    let mut prime_list = vec![true; 1_000_001];

    for i in 2..=(1_000_000_f64.sqrt() as usize) {
        if prime_list[i] {
            for j in i..=(1_000_000  / i) {
                prime_list[i * j] = false;
            }
        }
    }

    prime_list[0] = false;
    prime_list[1] = false;
    prime_list[2] = false;

    for n in input {
        if n == 0 {
            break;
        }

        let mut l = 0;
        let mut r = n;

        while l <= r {
            if prime_list[l] && prime_list[r] {
                break;
            }

            l += 1;
            r -= 1;
        }

        writeln!(output, "{} = {} + {}", n, l ,r).unwrap();
    }

    print!("{output}");
}