// 특정 대문자를 소문자로 바꾸기

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let mut a = input.next().unwrap().to_string();

    while let Some(b) = input.next() {
        let b = b.parse::<char>().unwrap();
        unsafe {
            solution(&mut a, b);
        }
    }

    println!("{}", a);
}

unsafe fn solution(a: &mut String, b: char) {
    let vec = a.as_mut_vec();
    for i in vec {
        if 65 <= *i && *i <= 90 {
            if *i == b as u8 {
                *i += 32;
            }
        }
    }
}
