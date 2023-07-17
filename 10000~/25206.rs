// 너의 평점은

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.lines();
    let mut sum = 0.0;
    let mut div = 0.0;

    for line in input {
        let mut temp = line.split_ascii_whitespace().skip(1);
        let a = temp.next().unwrap().parse::<f64>().unwrap();
        let b = match temp.next().unwrap() {
            "A+" =>	4.5,
            "A0" =>	4.0,
            "B+" =>	3.5,
            "B0" =>	3.0,
            "C+" =>	2.5,
            "C0" =>	2.0,
            "D+" =>	1.5,
            "D0" =>	1.0,
            "F"  => 0.0,
            _ => continue,
        };
        div += a;
        sum += a * b;
    }

    print!("{}", sum / div);
}