// 도비의 영어 공부

use std::io::{self, BufWriter, Read, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut output = BufWriter::new(stdout.lock());

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    loop {
        let mut line = input
            .next()
            .unwrap()
            .split_ascii_whitespace();

        match line.next().unwrap().parse().unwrap() {
            '#' => break,
            a   => {
                let mut count = 0;
                for c in line.collect::<String>().chars() {
                    if c.to_ascii_lowercase() == a {
                        count += 1;
                    }
                }

                writeln!(output, "{} {}", a, count).unwrap();
            }
        }
    }
}
