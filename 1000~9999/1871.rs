// 좋은 자동차 번호판

use std::io;

fn main() {
    let n: u8 = input().parse().unwrap();

    for _ in 0..n {
        let data = input();

        let result = if is_good(data) {
            "nice"
        } else {
            "not nice"
        };

        println!("{}", result);
    }
}

fn is_good(data: String) -> bool {
    let left  = &data[..3];
    let right = &data[4..];

    let left_to_int  = alphabet_to_decimal(left);
    let right_to_int: i32 = right.parse().unwrap();

    let dif = left_to_int - right_to_int;

    if
        -100    <=    dif && dif    <=    100
    {
        true
    } else {
        false
    }
}

fn alphabet_to_decimal(alphabet: &str) -> i32 {
    let mut result = 0;
    let mut x = alphabet.len() as u32;

    for c in alphabet.chars() {
        x -= 1;
        result += (c as i32 - 65) * 26_i32.pow(x);
    }

    result
}

fn input() -> String {
    let mut data = String::new();

    io::stdin().read_line(&mut data).unwrap();

    data.trim().to_string()
}
