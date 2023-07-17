// 수학은 체육과목 입니다

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    print!("{}", input.trim().parse::<u32>().unwrap() * 4);
}
