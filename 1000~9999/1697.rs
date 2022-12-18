// 숨바꼭질

use std::collections::VecDeque;

macro_rules! input_line {
    ($($t: ty), +) => ({
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_ascii_whitespace();
        ($(iter.next().unwrap().parse::<$t>().unwrap()), +)
    });
}

fn main() {
    let (n, k) = input_line!(usize, usize);

    let result = solution(n, k);
    println!("{result}");
}

fn solution(n: usize, k: usize) -> i32 {
    let mut v = [-1; 200000];
    let mut queue = VecDeque::from([(n, 0)]);

    while let Some((a, c)) = queue.pop_front() {
        v[a] = c;
        if a == k { break; }
        let a = a as i32;
        for i in [a - 1, a + 1, 2 * a] {
            if 0 <= i && i < v.len() as i32 {
                let i = i as usize;
                if v[i] == -1 {
                    queue.push_back((i, c + 1));
                }
            }
        }
    }

    v[k]
}
