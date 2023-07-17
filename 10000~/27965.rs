// N결수

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let k: usize = input.next().unwrap().parse().unwrap();

    let mut num = 0;

    for i in 1..=n {
        num = (num * pow(10, get_digit(i)) + i) % k;
    }

    println!("{num}");
}

fn get_digit(mut n: usize) -> usize {
    let mut count = 0;
    while 0 < n {
        n /= 10;
        count += 1;
    }

    count
}

fn pow(mut base: usize, mut exp: usize) -> usize {
    let mut result = 1;
    while 0 < exp {
        if exp & 1 != 0 {
            result *= base;
        }
        exp >>= 1;
        base *= base;
    }

    result
}