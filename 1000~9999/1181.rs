// 단어 정렬

use std::fmt::Write;

macro_rules! input_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap() ),+)
})}

fn main() {
    let n = input_line!(u16);
    let mut v = Vec::new();
    
    for _ in 0..n {
        v.push(input_line!(String));
    }

    print!("{}", solution(v));
}

#[inline]
fn solution(mut v: Vec<String>) -> String {
    let mut output = String::new();

    v.sort_unstable_by(|a, b|
        a.len().cmp(&b.len()).then(a.cmp(&b))
    );

    v.dedup();

    v.iter().for_each(|i|
        writeln!(output, "{}", i).unwrap()
    );

    output
}
