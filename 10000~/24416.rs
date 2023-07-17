// 알고리즘 수업 - 피보나치 수 1

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let n: u32 = input.trim().parse().unwrap();
    let mut dp = vec![1, 1];
    let mut cnt = 0;

    for _ in 3..=n {
        dp.push(dp[dp.len() - 1] + dp[dp.len() - 2]);
        cnt += 1;
    }

    print!("{} {}", dp[dp.len() - 1], cnt);
}