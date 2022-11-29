// 요세푸스 문제 0

use std::collections::VecDeque;
use std::fmt::{Display, Formatter, Error};

macro_rules! input_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap() ),+)
})}

struct Josephus(Vec<usize>);

impl Display for Josephus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut comma_separated = String::new();
        comma_separated.push('<');

        for num in &self.0[0..self.0.len() - 1] {
            comma_separated.push_str(&num.to_string());
            comma_separated.push_str(", ");
        }

        comma_separated.push_str(&self.0[self.0.len() - 1].to_string());
        comma_separated.push('>');
        write!(f, "{}", comma_separated)
    }
}

fn main() {
    let (n, k) = input_line!(usize, usize);
    let result = solution(n, k);
    println!("{}", result);
}

fn solution(n: usize, k: usize) -> Josephus {
    let mut josephus = Vec::new();
    let mut list = VecDeque::new();
    let mut i = 0;
    
    for i in 1..=n {
        list.push_back(i);
    }

    while 0 < list.len() {
        i += k - 1;
        i %= list.len();
        josephus.push(list.remove(i).unwrap());
    }

    Josephus(josephus)
}
