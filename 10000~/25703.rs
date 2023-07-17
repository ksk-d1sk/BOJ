// 포인터 공부

use std::io;

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: u8 = input.trim().parse().unwrap();

    output.push_str("int a;\n");

    for i in 1..=n {
        output.push_str("int ");

        for _ in 0..i {
            output.push('*');
        }

        output.push_str(&to_string(i));
        output.push_str(" = &");
        output.push_str(&to_string(i - 1));
        output.push_str(";\n");
    }

    print!("{output}");
}

fn to_string(n: u8) -> String {
    if n == 0 {
        String::from("a")
    } else if n == 1 {
        String::from("ptr")
    } else {
        let mut str = String::from("ptr");
        str.push_str(&n.to_string());
        str
    }
}