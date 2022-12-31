// 집합

use std::io::{self, BufRead, Write, BufWriter};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut output = BufWriter::new(stdout.lock());

    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let mut iter = line.split_ascii_whitespace();

    macro_rules! iter_next {
        () => { iter.next().unwrap() };
        ( $t:ty ) => { iter.next().unwrap().parse::<$t>().unwrap() };
    }

    let mut set = 0_u32;

    for _ in 0..iter_next!(u32) {
        line.clear();
        stdin.read_line(&mut line).unwrap();
        iter = line.split_ascii_whitespace();
        match iter_next!() {
            "add"    => set |= 1 << iter_next!(u32),
            "remove" => set &= !(1 << iter_next!(u32)),
            "check"  => writeln!(output, "{}", (set >> iter_next!(u32)) & 1).unwrap(),
            "toggle" => set ^= 1 << iter_next!(u32),
            "all"    => set = (1 << 21) - 1,
            "empty"  => set = 0,
            _ => panic!(),
        }
    }
}
