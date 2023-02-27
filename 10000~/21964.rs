// 선린인터넷고등학교 교가

use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let input = buf
        .split_ascii_whitespace()
        .skip(1)
        .next()
        .unwrap();

    let len = input.len();
    println!("{}", &input[len-5..]);
}
