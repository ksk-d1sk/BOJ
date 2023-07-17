// 색종이

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! input {
        ( $( $t:ty ),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()), +) };
    }

    let mut v = [[false; 100]; 100];

    for _ in 0..input!(u8) {
        let (x, y) = input!(usize, usize);
        for i in x..(x + 10) {
            for j in y..(y + 10) {
                v[i][j] = true;
            }
        }
    }

    println!("{}", v.iter().flatten().filter(|&&e| e).count());
}