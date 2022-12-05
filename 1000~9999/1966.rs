// 프린터 큐

use std::io::{self, Read};
use std::fmt::Write;
use std::collections::{BinaryHeap, VecDeque};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().map(|i| i.parse::<usize>().unwrap());

    let mut output = String::new();
    let t = input.next().unwrap();
    
    for _ in 0..t {
        let (n, m) = (input.next().unwrap(), input.next().unwrap());
        let mut queue = VecDeque::new();
        let mut b_heap = BinaryHeap::new();
        let mut count = 0;

        for i in 0..n {
            let importance = input.next().unwrap();
            b_heap.push(importance);
            queue.push_back((i, importance));
        }

        while let Some(x) = queue.pop_front() {
            if x.1 == *b_heap.peek().unwrap() {
                count += 1;
                b_heap.pop();
                if x.0 == m {
                    break;
                }
            } else {
                queue.push_back(x);
            }
        }

        writeln!(output, "{}", count).unwrap();
    }

    print!("{}", output);
}
