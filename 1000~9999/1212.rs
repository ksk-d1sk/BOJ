// 8진수 2진수

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    println!("{}", solution(input));
}

fn solution(octal: &str) -> String {
    let chars = octal.chars();
    let mut buf = String::new();

    for c in chars {
        let mut octal = c as u8 - b'0';
        for _ in 0..3 {
            if octal & 4 == 4 {
                buf.push('1');
            } else {
                buf.push('0');
            }
            octal <<= 1;
        }
    }

    remove_front_zero(buf)
}

fn remove_front_zero(string: String) -> String {
    let mut i = 0;

    for &a in string.as_bytes() {
        if a == b'1' { break; }
        i += 1;
    }

    if i == string.len() {
        i -= 1;
    }

    string[i..string.len()].to_string()
}
