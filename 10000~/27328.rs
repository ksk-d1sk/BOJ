// 三方比較 (Three-Way Comparison)

use std::io::{self, Read};

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! get {
        ( $( $t:ty ),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()), +) };
    }

    let (a, b) = get!(u16, u16);

    print!(
        "{}",
        if a > b {
            1
        } else
        if a == b {
            0
        } else {
            -1
        }
    );
}
