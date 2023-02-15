// 팩토리얼 0의 개수

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: usize = input.trim().parse().unwrap();

    let mut two_count = 0;
    let mut five_count = 0;

    for mut i in 2..=n {
        while i % 5 == 0 {
            five_count += 1;
            i /= 5;
        }

        while i & 1 == 0 {
            two_count += 1;
            i /= 2;
        }
    }

    println!("{}", two_count.min(five_count));
}
