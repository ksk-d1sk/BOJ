// 알파벳 찾기

use std::io;

fn main() {
    let mut word = String::new();
    io::stdin().read_line(&mut word).unwrap();
    let word = word.trim();
    let mut alphabet = [-1; 26];
    for (i, &c) in word.as_bytes().iter().enumerate() {
        if alphabet[c as usize - 97] == -1 {
            alphabet[c as usize - 97] = i as i8;
        }
    }
    alphabet.iter()
        .for_each(|i| print!("{} ", i));
}
