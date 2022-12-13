// 듣보잡

use std::io::{self, Read};
use std::fmt::Write;
use std::collections::BTreeSet;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<u32>().unwrap();
    let m = input.next().unwrap().parse::<u32>().unwrap();

    let mut deud = Vec::new();
    let mut bo = Vec::new();

    for _ in 0..n {
        deud.push(input.next().unwrap());
    }

    for _ in 0..m {
        bo.push(input.next().unwrap());
    }

    let result = if m < n {
        solution(deud, bo)
    } else {
        solution(bo, deud)
    };

    print!("{result}");
}

fn solution(mut long_v: Vec<&str>, short_v: Vec<&str>) -> String {
    let mut buf = String::new();
    let mut set = BTreeSet::new();

    long_v.sort_unstable();

    for word in short_v {
        let result = long_v.binary_search(&word);
        if let Ok(_) = result {
            set.insert(word);
        }
    }

    writeln!(buf, "{}", set.len()).unwrap();
    for word in set {
        writeln!(buf, "{}", word).unwrap();
    }

    buf
}
