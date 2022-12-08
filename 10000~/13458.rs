// 시험 감독

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().map(|i| i.parse::<i64>().unwrap());

    let n = input.next().unwrap();
    let mut v = Vec::new();

    for _ in 0..n {
        v.push(input.next().unwrap());
    }

    let (b, c) = (input.next().unwrap(), input.next().unwrap());

    let result = solution(v, b, c);
    println!("{result}");
}

fn solution(v: Vec<i64>, b: i64, c: i64) -> i64 {
    let mut answer = v.len() as i64;

    v.into_iter()
        .filter_map(|i|
            if 0 < i - b {
                Some(i - b)
            } else { None }
        )
        .for_each(|i| {
            answer += i / c;
            if i % c != 0 {
                answer += 1;
            }
        });

    answer
}
