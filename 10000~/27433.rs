// 팩토리얼 2

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse().unwrap();
    
    println!("{}", facto(n));
}

fn facto(n: usize) -> usize {
    if n < 2 {
        1
    } else {
        n * facto(n - 1)
    }
}