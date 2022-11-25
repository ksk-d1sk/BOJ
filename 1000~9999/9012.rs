// 괄호

use std::io;
use std::fmt::Write;

fn main() {
    let mut output = String::new();
    let t: u32 = input().parse().unwrap();

    for _ in 0..t {
        let data = input();
        writeln!(
            &mut output, "{}",
            if solution(data) { "YES" } else { "NO" }
        ).unwrap();
    }

    println!("{}", output);
}

fn solution(data: String) -> bool {
    let mut answer = true;
    let mut count = 0;

    for c in data.chars() {
        match c {
            '(' => count += 1,
            ')' => {
                if count == 0 {
                    answer = false;
                    break;
                } else {
                    count -= 1;
                }
            },
            _   => panic!(),
        }
    }

    if count != 0 {
        answer = false;
    }

    answer
}

fn input() -> String {
    let mut data = String::new();

    io::stdin().read_line(&mut data).unwrap();

    data.trim().to_string()
}
