// 전자레인지

use std::io::{self, Write, BufWriter, BufRead};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout);

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let input = input.trim_end().parse::<u16>().unwrap();

    match solution(input) {
        Ok(ans) => writeln!(stdout, "{} {} {}", ans.0, ans.1, ans.2).unwrap(),
        Err(e) => writeln!(stdout, "{}", e).unwrap(),
    }
}

fn solution(mut t: u16) -> Result<(u16, u16, u16), i8> {
    if t % 10 == 0 {
        let a = t / 300;
        t %= 300;

        let b = t / 60;
        t %= 60;

        let c = t / 10;

        Ok((a, b, c))
    } else {
        Err(-1)
    }
}
