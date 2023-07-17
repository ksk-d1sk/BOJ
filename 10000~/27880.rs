// Gahui and Soongsil University station

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! get {
        ($t:ty) => { input.next().unwrap().parse::<$t>().unwrap() };
        ()      => { input.next().unwrap() }
    }

    let mut sum = 0;

    for _ in 0..4 {
       sum += if get!() == "Es" {
           get!(u32) * 21
       } else {
           get!(u32) * 17
       };
    }

    print!("{sum}")
}