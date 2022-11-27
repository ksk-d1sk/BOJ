// 크냐?

use std::fmt::Write;

macro_rules! input_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();

    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn main() {
    let output = {
        let mut buf = String::new();

        loop {
            let (a, b) = input_line!(u32, u32);

            if a == 0 && b == 0 {
                break buf;
            }

            writeln!(
                buf, "{}",
                if a > b { "Yes" } else { "No" }
            ).unwrap();
        }
    };

    print!("{}", output);
}
