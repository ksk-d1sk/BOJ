// 키로거

use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! next {
        () => { input.next().unwrap() };
        ( $t:ty ) => { input.next().unwrap().parse::<$t>().unwrap() };
    }

    for _ in 0..next!(usize) {
        let mut keylog = VecDeque::new();
        let mut n = 0;

        for c in next!().chars() {
            match c {
                '<' => if n != 0 {
                    n -= 1;
                }
                '>' => if n != keylog.len() {
                    n += 1;
                }
                '-' => if n != 0 {
                    keylog.remove(n - 1);
                    n -= 1;
                }
                _   => {
                    keylog.insert(n, c);
                    n += 1;
                }
            }
        }

        for c in keylog {
            output.push(c);
        }
        output.push('\n');
    }

    print!("{output}");
}
