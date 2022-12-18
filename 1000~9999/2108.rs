// 통계학

use std::io::{self, Read};
use std::collections::BTreeMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let mut v = Vec::new();

    for _ in 0..n {
        v.push(input.next().unwrap().parse::<i32>().unwrap());
    }

    v.sort_unstable();
    let rv = RepresentativeValue(v);

    println!(
        "{}\n{}\n{}\n{}",
        rv.mean(),
        rv.median(),
        rv.mode(),
        rv.range(),
    )
}

struct RepresentativeValue(Vec<i32>);

impl RepresentativeValue {
    #[inline]
    fn mean(&self) -> i32 {
        let mut ans = 0.0;

        for &i in &self.0 {
            ans += i as f64;
        }

        (ans / self.0.len() as f64).round() as i32
    }

    #[inline]
    fn median(&self) -> i32 {
        self.0[self.0.len() / 2]
    }

    #[inline]
    fn mode(&self) -> i32 {
        let mut map = BTreeMap::new();
        for &i in &self.0 {
            map.entry(i).and_modify(|i| *i += 1).or_insert(1);
        }

        let mut iter = map.iter().filter(|(_, v)| *v == map.values().max().unwrap()).cycle();

        iter.next();
        *iter.next().unwrap().0
    }

    #[inline]
    fn range(&self) -> i32 {
        self.0.iter().max().unwrap() - self.0.iter().min().unwrap()
    }
}
