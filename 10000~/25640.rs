// MBTI

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let mut count = 0;
    let my_mbti = input.next().unwrap();
    input.next();

    while let Some(friends_mbti) = input.next() {
        if my_mbti == friends_mbti {
            count += 1;
        }
    }

    println!("{}", count);
}
