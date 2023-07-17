// 2×n 타일링 2

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: usize = input.trim().parse().unwrap();
    let mut dp = vec![0_u16; n.max(2)];

    dp[0] = 1;
    dp[1] = 3;

    for i in 2..n {
        dp[i] = (dp[i - 1] + dp[i - 2] * 2) % 10_007;
    }

    println!("{}", dp[n - 1]);
}