// 연세대학교

use std::io;

fn main() {
    let n = input();
    println!(
        "{}",
        if n == 1 {
            "Leading the Way to the Future"
        } else {
            "YONSEI"
        }
    );
}

fn input() -> u8 {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}
