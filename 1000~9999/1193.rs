// 분수찾기

use std::io;

fn main() {
    let x = input();

    println!("{}", solution(x));
}

fn solution(x: u32) -> String {
    let mut cp_x = x;
    let mut count = 1;
    let mut a = 1;

    while cp_x > a {
        cp_x -= a;
        count += a;
        a += 1;
    }

    let temp = x - count;

    let (left, right) = if a % 2 == 1 {
        (a - temp, 1 + temp)
    } else {
        (1 + temp, a - temp)
    };

    format!("{}/{}", left, right)
}

fn input() -> u32 {
    let mut data = String::new();

    io::stdin().read_line(&mut data).unwrap();

    data.trim().parse().unwrap()
}
