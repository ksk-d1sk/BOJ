// 별 찍기 - 10

use std::io;

fn main() {
    let mut input = String::new();
    let mut output = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    for x in 0..n {
        for y in 0..n {
            if blank_check(n, x, y) {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    println!("{output}");
}

fn blank_check(n: usize, x: usize, y: usize) -> bool {
    if n == 1 {
        true
    } else if x / (n / 3) % 3 == 1 && y / (n / 3) % 3 == 1 {
        false
    } else {
        blank_check(n / 3, x, y)
    }
}
