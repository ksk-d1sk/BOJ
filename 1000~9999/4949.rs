// 균형잡힌 세상

use std::io;
use std::fmt::Write;
use std::collections::VecDeque;

fn main() {
    let mut output = String::new();

    while let Some(line) = input() {
        writeln!(
            output, "{}",
            if solution(line) { "yes" } else { "no" }
        ).unwrap();
    }

    print!("{}", output);
}

fn solution(line: String) -> bool {
    let mut check = true;
    let mut stack = VecDeque::new();

    for c in line.chars() {
        if c == '(' || c == '[' {
            stack.push_back(c);
        } else if c == ')' {
            if stack.is_empty() || stack.pop_back().unwrap() != '(' {
                check = false;
                break;
            }
        } else if c == ']' {
            if stack.is_empty() || stack.pop_back().unwrap() != '[' {
                check = false;
                break;
            }
        }
    }

    if !stack.is_empty() {
        check = false;
    }

    check
}

fn input() -> Option<String> {
    let mut data = String::new();

    io::stdin().read_line(&mut data).unwrap();

    let a = data.trim_end();
    let mut answer = Some(a.to_string());

    if a == "." {
        answer = None;
    }

    answer
}
