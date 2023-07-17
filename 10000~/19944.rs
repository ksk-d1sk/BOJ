// 뉴비의 기준은 뭘까?

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! input {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()), +) };
    }

    let (n, m) = input!(u16, u16);

    print!(
        "{}",
        if m < 3 {
            "NEWBIE!"
        } else if m <= n {
            "OLDBIE!"
        } else {
            "TLE!"
        }
    )
}