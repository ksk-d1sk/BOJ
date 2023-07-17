// 배수 찾기

use std::io::Read;
use std::fmt::Write;

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<u16>);

    let n = input.next().unwrap();

    for i in input {
        if i != 0 {
            let _ = writeln!(
                output,
                "{} is{}a multiple of {}.",
                i, if i % n == 0 { " " } else { " NOT " }, n
            );
        }
    }

    print!("{output}");
}
