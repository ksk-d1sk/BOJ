// 母音を数える (Counting Vowels)

use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    print!("{}", input.split_ascii_whitespace().skip(1).next().unwrap().chars().filter(|&e| {
        match e {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        }
    }).count())
}
