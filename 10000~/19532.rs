// 수학은 비대면강의입니다

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! get {
        ( $( $t:ty ),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (a, b, c) = get!(i32, i32, i32);
    let (d, e, f) = get!(i32, i32, i32);

    print!(
        "{} {}",
        (c * e - b * f) / (a * e - b * d),
        (c * d - a * f) / (b * d - a * e)
    );
}