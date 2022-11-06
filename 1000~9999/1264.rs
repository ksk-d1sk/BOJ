// 모음의 개수

use std::io;

fn main() {
    while let Some(msg) = input() {
        let msg = msg.to_lowercase();
        let mut count = 0_u32;

        for c in msg.chars() {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    count += 1;
                },
                _ => (),
            }
        }

        println!("{}", count);
    }
}

fn input() -> Option<String> {
    let mut msg = String::new();

    io::stdin().read_line(&mut msg).unwrap();

    let msg = msg.trim().to_string();
    let mut answer = Some(msg.clone());

    if msg == "#" {
        answer = None;
    }

    answer
}
