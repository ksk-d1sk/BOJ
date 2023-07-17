// 가지 교배

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! get {
        () => { input.next().unwrap() };
        ( $( $t:ty ),+ ) => { ( $(input.next().unwrap().parse::<$t>().unwrap()),+ ) };
    }

    let mut answer = false;
    let n = get!(usize);
    let mut gaji_list = Vec::new();

    for _ in 0..n {
        gaji_list.push(get!());
    }

    let (m, k) = get!(usize, usize);

    for _ in 0..m {
        let mut check = true;

        for _ in 0..k {
            if gaji_list[get!(usize) - 1] == "P" {
                check = false;
            }
        }

        if check {
            answer = true;
            break;
        }
    }

    if answer {
        println!("W");
    } else {
        println!("P");
    }
}
