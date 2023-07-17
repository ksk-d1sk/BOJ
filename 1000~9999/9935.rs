// 문자열 폭발

use std::io::{self, Read};

fn main() {
    let mut input  = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let string = input.next().unwrap().as_bytes();
    let boom   = input.next().unwrap().as_bytes();
    let mut buf_stack = Vec::new();
    let mut cmp_stack = Vec::new();

    for c in string {
        buf_stack.push(*c);
        let mut check = true;

        for i in (0..boom.len()).rev() {
            if buf_stack.is_empty() {
                check = false;
                break;
            }

            let temp = buf_stack.pop().unwrap();
            cmp_stack.push(temp);

            if temp != boom[i] {
                check = false;
                break;
            }
        }

        if check {
            cmp_stack.clear();
        } else {
            while let Some(c) = cmp_stack.pop() {
                buf_stack.push(c);
            }
        }
    }

    if !buf_stack.is_empty() {
        println!("{}", String::from_utf8(buf_stack).unwrap());
    } else {
        println!("FRULA");
    }
}