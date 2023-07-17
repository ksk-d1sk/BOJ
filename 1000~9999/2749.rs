// 피보나치 수 3

use std::io;

const MOD: usize = 1_000_000;
const P: usize = 15 * MOD / 10;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut fibo = vec![0_usize; P];

    fibo[0] = 0;
    fibo[1] = 1;

    for i in 2..P {
        fibo[i] = fibo[i - 1] + fibo[i - 2];
        fibo[i] %= MOD;
    }

    println!("{}", fibo[n % P]);
}
