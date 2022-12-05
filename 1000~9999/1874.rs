// 스택 수열

use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().map(|i| i.parse::<usize>().unwrap());

    let n = input.next().unwrap();
    let mut v = VecDeque::new();

    for _ in 0..n {
        v.push_back(input.next().unwrap());
    }

    let result = solution(v);
    print!("{result}");
}

fn solution(mut v: VecDeque<usize>) -> String {
    let mut buf = String::new();
    let mut stack = VecDeque::new();
    let mut i = 1;

    while !v.is_empty() {
        if stack.back() == v.get(0) {
            buf.push_str("-\n");
            v.pop_front();
            stack.pop_back();
        } else if v[0] < i {
            v.clear();
            buf.clear();
            buf.push_str("NO\n");
        } else {
            buf.push_str("+\n");
            stack.push_back(i);
            i += 1;
        }
    }

    buf
}
