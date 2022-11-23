// 가장 큰 금민수

use std::io;

fn main() {
    let n = input();

    let result = (4..=n).rev().find(|&i| {
        let mut check = true;
        let mut x = i;

        while check && x != 0 {
            let temp = x % 10;
            if temp != 4 && temp != 7 {
                check = false;
            }
            x /= 10;
        }

        check
    }).unwrap();

    println!("{}", result);
}

fn input() -> u32 {
    let mut data = String::new();

    io::stdin().read_line(&mut data).unwrap();

    data.trim().parse().unwrap()
}
