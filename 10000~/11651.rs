// 좌표 정렬하기 2

use std::io::{self, Read};
use std::fmt::Write;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().map(|i| i.parse::<i32>().unwrap());

    let n = input.next().unwrap() as u32;
    let mut coordinates = Vec::new();
    for _ in 0..n {
        let data = (input.next().unwrap(), input.next().unwrap());
        coordinates.push(data);
    }

    let result = solution(coordinates);
    print!("{}", result);
}

fn solution(mut coordinates: Vec<(i32, i32)>) -> String {
    let mut buf = String::new();

    coordinates.sort_unstable_by(|&(x1, y1), &(x2, y2)|
        if y1 == y2 {
            x1.cmp(&x2)
        } else {
            y1.cmp(&y2)
        }
    );

    for (x, y) in coordinates {
        writeln!(buf, "{} {}", x, y).unwrap();
    }

    buf
}
