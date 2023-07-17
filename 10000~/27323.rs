// 직사각형

use std::io::{self, Read};

fn main() {
    let mut input  = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! get {
        ( $( $t:ty ),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()), +) };
    }

    let (a, b) = get!(u16, u16);

    print!("{}", a * b);
}