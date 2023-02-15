// 숫자세는 양 (Large)

use std::io::{self, Read, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut output = BufWriter::new(stdout);

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .map(str::parse)
        .flatten();

    for i in 1..=input.next().unwrap() {

        match solution(input.next().unwrap()) {
            Ok(ans) => writeln!(output, "Case #{}: {}", i, ans).unwrap(),
            Err(e)  => writeln!(output, "Case #{}: {}", i, e).unwrap(),
        }
    }
}

fn solution(n: u64) -> Result<u64, &'static str> {
    if n == 0 {
        Err("INSOMNIA")
    } else {
        let mut find = [true; 10];
        let mut i = 0;
        while 0 < find.iter().filter(|e| **e).count() {
            i += 1;
            for c in (n * i).to_string().as_bytes() {
                find[(c - b'0') as usize] = false;
            }
        }
        Ok(n * i)
    }
}
