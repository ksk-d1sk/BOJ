// 바이러스

use std::io::{self, Read};

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! input {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()), +) };
    }

    let mut v: Vec<(bool, Vec<usize>)> = vec![(false, Vec::new()); input!(usize)];

    for _ in 0..input!(usize) {
        let (l, r) = input!(usize, usize);
        v[l - 1].1.push(r - 1);
        v[r - 1].1.push(l - 1);

    }

    let mut stack = vec![0];

    while let Some(i) = stack.pop() {
        if !v[i].0 {
            v[i].0 = true;
            for link in &v[i].1 {
                stack.push(*link);
            }
        }
    }

    println!("{}", v.iter().filter(|e| e.0).count() - 1);
}