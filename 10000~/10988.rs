// 팰린드롬인지 확인하기

use std::io;

fn main() {
    let mut input  = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let word = input.trim().as_bytes();

    println!("{}", is_palindrome(word));
}

fn is_palindrome(word: &[u8]) -> i8 {
    if recv(word, 0, word.len() - 1) { 1 } else { 0 }
}

fn recv(word: &[u8], l: usize, r: usize) -> bool {
    if l >= r {
        true
    } else if word[l] != word[r] {
        false
    } else {
        recv(word, l + 1, r - 1)
    }
}