// 별 찍기 - 19

use std::io;

fn main() {
    let mut input = String::new();
    let mut output = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n = (input.trim().parse::<u16>().unwrap() - 1) * 4 + 1;

    for x in 0..n {
        for y in 0..n {
            if blank_check(x, y, 0, n - 1) {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    print!("{output}")
}

fn blank_check(x: u16, y: u16, start: u16, end: u16) -> bool {
    if x < start || end < x ||
       y < start || end < y {
        false
    } else
    if x == start || x == end ||
       y == start || y == end {
        true
    } else {
        blank_check(x, y, start + 2, end - 2)
    }
}
