// 약수들의 합

use std::io::{self, Read};
use std::fmt::{Display, Write};

struct BOJ9506 {
    n: i32,
    factor: Vec<i32>,
    check: bool,
}

impl BOJ9506 {
    fn new(n: i32) -> Self {
        Self { n, factor: Vec::new(), check: true }
    }

    fn solve(&mut self) {
        for i in 1..=(self.n / 2) {
            if self.n % i == 0 {
                self.factor.push(i);
            }
        }

        self.check = self.factor.iter().sum::<i32>() == self.n;
    }
}

impl Display for BOJ9506 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.check {
            let factor_sum = self.factor
                .iter()
                .map(i32::to_string)
                .collect::<Vec<_>>()
                .join(" + ");

            write!(f, "{} = {}", self.n, factor_sum)
        } else {
            write!(f, "{} is NOT perfect.", self.n)
        }
    }    
}

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input
        .split_ascii_whitespace()
        .flat_map(str::parse::<i32>);

    for n in input {
        if n == -1 { break; }
        let mut boj_9506 = BOJ9506::new(n); 
        boj_9506.solve();
        writeln!(output, "{boj_9506}").unwrap();
    }

    print!("{output}");
}

#[cfg(test)]
mod tests {
    use crate::BOJ9506;
    use std::fmt::Write;

    #[test]
    fn test1() {
        let mut output = String::new();
        let n = 6;
        let mut boj_9506 = BOJ9506::new(n);

        boj_9506.solve();

        write!(output, "{boj_9506}").unwrap();

        assert_eq!(output, "6 = 1 + 2 + 3");
    }

    #[test]
    fn test2() {
        let mut output = String::new();
        let n = 12;
        let mut boj_9506 = BOJ9506::new(n);

        boj_9506.solve();

        write!(output, "{boj_9506}").unwrap();

        assert_eq!(output, "12 is NOT perfect.");
    }

    #[test]
    fn test3() {
        let mut output = String::new();
        let n = 28;
        let mut boj_9506 = BOJ9506::new(n);

        boj_9506.solve();

        write!(output, "{boj_9506}").unwrap();

        assert_eq!(output, "28 = 1 + 2 + 4 + 7 + 14");
    }
}