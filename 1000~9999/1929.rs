// 소수 구하기

use std::io;
use std::cmp;
use std::fmt::Write;

fn main() {
    let (m, n);
    let temp = line_input();

    (m, n) = (temp[0], temp[1]);

    solution(m, n);
}

fn solution(m: usize, n: usize) {
    let mut output = String::new();
    let mut prime_no = vec![true; n + 1];

    (2..=n).for_each(|i| {
        if prime_no[i] {
            (i..=(n / i)).for_each(|j| {
                prime_no[i * j] = false;
            })
        }
    });

    (cmp::max(m, 2)..=n).filter(|i| prime_no[*i]).for_each(|i| {
        writeln!(output, "{}", i).unwrap();
    });

    println!("{}", output);
}

fn line_input() -> Vec<usize> {
    let mut datas = String::new();

    io::stdin().read_line(&mut datas).unwrap();

    datas.split_whitespace().map(|data| {
        data.parse().unwrap()
    }).collect()
}
