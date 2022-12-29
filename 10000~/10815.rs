// 숫자 카드

use std::io::{self, Read};
use std::fmt::Write;

fn main() {
    let mut output = String::new();
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .map(str::parse::<i32>)
        .flatten();

    let mut my_card: Vec<i32> = (0..input.next().unwrap())
        .map(|_| input.next().unwrap())
        .collect();

    my_card.sort_unstable();

    for i in input.skip(1) {
        write!(output, "{} ",
            if my_card.binary_search(&i).is_ok() { 1 } else { 0 }
        ).unwrap()
    }

    println!("{output}");
}
