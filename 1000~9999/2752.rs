// 세수정렬

use std::collections::BTreeSet;

macro_rules! input_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap() ),+)
})}

fn main() {
    let mut btset = BTreeSet::new();
    let (a, b, c) = input_line!(u32, u32, u32);

    btset.insert(a);
    btset.insert(b);
    btset.insert(c);

    btset.iter().for_each(|i| print!("{} ", i));
}
