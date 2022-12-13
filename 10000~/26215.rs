// 눈 치우기

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().map(|i| i.parse::<u16>().unwrap());

    let n = input.next().unwrap();
    let mut v = Vec::new();

    for _ in 0..n {
        v.push(input.next().unwrap());
    }

    let result = solution(v);
    println!("{result}")
}

fn solution(mut v: Vec<u16>) -> i16 {
    let mut count = 0;

    while snow_clearing(&mut v) && 0 <= count {
        count += 1;
        if 1440 < count {
            count = -1;
        }
    }

    count
}

fn snow_clearing(v: &mut Vec<u16>) -> bool {
    let mut did_clean = false;
    v.sort_unstable_by(|a, b| b.cmp(a));
    if v[0] != 0 {
        did_clean = true;
        v[0] -= 1;
        if let Some(a) = v.get_mut(1) {
            if *a != 0 {
                *a -= 1;
            }
        }
    }
    did_clean
}
