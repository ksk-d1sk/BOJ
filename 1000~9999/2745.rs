// 진법 변환

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! get {
        () => { input.next().unwrap() };
        ( $t:ty ) => { input.next().unwrap().parse::<$t>().unwrap() };
    }

    let mut answer = 0;
    let n    = get!().as_bytes();
    let base = get!(i32);

    for (index, &value) in n.iter().rev().enumerate() {
        answer += match value {
            b'0'..=b'9' => { (value - b'0'     ) as i32 * base.pow(index as u32) },
            b'A'..=b'Z' => { (value - b'A' + 10) as i32 * base.pow(index as u32) },
            _ => panic!(),            
        };
    }

    print!("{answer}");
}