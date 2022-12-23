// 럭비 클럽

use std::fmt::Write;

macro_rules! input_line {
    ($($t: ty), +) => ({
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_ascii_whitespace();
        ($(iter.next().unwrap().parse::<$t>().unwrap()), +)
    });
}

fn main() {
    let mut output = String::new();

    print!(
        "{}",
        loop {
            let (name, age, weight) = input_line!(String, u32, u32);
            if &name[..] == "#" {
                break output;
            }
            writeln!(
                output, "{} {}",
                name,
                if 17 < age || 80 <= weight { "Senior" } else { "Junior" }
            ).unwrap();
        }
    )
}
