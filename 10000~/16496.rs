// 큰 수 만들기

use std::io::Read;
use std::fmt::Write;

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut v: Vec<&str> = input.split_ascii_whitespace().skip(1).collect();

    v.sort_unstable_by(|&a, &b| {
        format!("{}{}", b, a).cmp(&format!("{}{}", a, b))
    });

    for elem in v {
        if output.len() > 0 || elem != "0" {
            let _ = write!(output, "{}", elem);
        }
    }

    if !output.is_empty() {
        print!("{output}");
    } else {
        print!("0");
    }
}