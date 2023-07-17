// Fibonacci

use std::io::Read;
use std::fmt::Write;

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    for n in input.split_ascii_whitespace().flat_map(str::parse::<u64>) {
        writeln!(output, "{}", fibo(n, 10000)).unwrap();
    }

    print!("{output}");
}

fn fibo(mut n: u64, m: u64) -> u64 {
    let mut ans_metrix = [
        [1, 0],
        [0, 1],
    ];

    let mut metrix = [
        [1, 1],
        [1, 0],
    ];

    while n > 0 {
        if n & 1 == 1 {
            ans_metrix = metrix_mul(ans_metrix, metrix, m);
        }
        metrix = metrix_mul(metrix, metrix, m);
        n >>= 1;
    }

    ans_metrix[0][1]
}

fn metrix_mul(a: [[u64; 2]; 2], b: [[u64; 2]; 2], m: u64) -> [[u64; 2]; 2] {
    let mut answer = [[0; 2]; 2];

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                answer[i][j] += a[i][k] * b[k][j];
                answer[i][j] %= m;
            }
        }
    }

    answer
}