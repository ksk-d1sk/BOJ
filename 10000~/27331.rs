// 2 桁の整数 (Two-digit Integer)

use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    print!("{}{}", input.next().unwrap(), input.next().unwrap());
}
