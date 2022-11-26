// 카드2

use std::io;
use std::collections::VecDeque;

fn main() {
    let n = input();
    
    println!("{}", solution(n));
}

fn solution(n: u32) -> u32 {
    let mut cards = VecDeque::new();

    for i in 1..=n {
        cards.push_back(i);
    }

    while 1 < cards.len() {
        cards.pop_front();
        let temp = cards.pop_front().unwrap();
        cards.push_back(temp);
    }

    cards.pop_front().unwrap()
}

fn input() -> u32 {
    let mut data = String::new();

    io::stdin().read_line(&mut data).unwrap();

    data.trim().parse().unwrap()
}
