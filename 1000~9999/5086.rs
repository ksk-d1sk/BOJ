// 배수와 약수

use std::io::{self, Read};

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ( $(input.next().unwrap().parse::<$t>().unwrap()),+ ) };
    }

    loop {
        let (l, r) = get!(u16, u16);

        if l + r == 0 {
            break;
        }

        if l > r {
            if l % r == 0 {
                output.push_str("multiple\n");
            } else {
                output.push_str("neither\n");
            }
        } else {
            if r % l == 0 {
                output.push_str("factor\n");
            } else {
                output.push_str("neither\n");
            }
        }
    }

    print!("{output}");
}