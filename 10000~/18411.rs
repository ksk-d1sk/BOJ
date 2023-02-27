// 試験 (Exam)

use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut input: Vec<u16> = buf
        .split_ascii_whitespace()
        .map(|e| e.parse().unwrap())
        .collect();

    print!(
        "{}",
        input.select_nth_unstable(0).2.iter().sum::<u16>()
    );
}
