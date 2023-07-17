// Pyramids

use std::io::Read;
use std::fmt::Write;

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    for n in input.split_ascii_whitespace().flat_map(str::parse) {
        if n != 0 {
            let _ = writeln!(output, "{}", (1..=n).sum::<u32>());
        }
    }

    print!("{output}");
}
