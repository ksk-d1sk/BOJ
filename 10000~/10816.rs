// 숫자 카드 2

use std::io::{self, Read};
use std::collections::HashMap;
use std::fmt::Write;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>().unwrap();
    let mut my_card = HashMap::new();
    for _ in 0..n {
        let data = input.next().unwrap().parse::<i32>().unwrap();
        if let Some(value) = my_card.get(&data) {
            my_card.insert(data, value + 1);
        } else {
            my_card.insert(data, 1_u32);
        }
    }

    let m = input.next().unwrap().parse::<usize>().unwrap();
    let mut compare_card = Vec::new();
    for _ in 0..m {
        compare_card.push(input.next().unwrap().parse::<i32>().unwrap());
    }

    let result = solution(my_card, compare_card);
    println!("{}", result);
}

#[inline]
fn solution(my_card: HashMap<i32, u32>, compare_card: Vec<i32>) -> String {
    let mut answer = String::new();

    for compare in compare_card {
        write!(answer, "{} ", my_card.get(&compare).unwrap_or(&0)).unwrap();
    }

    answer
}
