// 닉네임에 갓 붙이기

use std::io::{self, Read};

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();
    let n: u8 = input.next().unwrap().parse().unwrap();

    for _ in 0..n {
        let line = input.next().unwrap();
        output.push_str("god");
        for word in line.split_ascii_whitespace().skip(1) {
            output.push_str(word);
        }
        output.push('\n');
    }

    print!("{output}");
}
