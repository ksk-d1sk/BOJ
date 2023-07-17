// 꼬마 정민

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    print!(
        "{}",
        input.split_ascii_whitespace()
            .flat_map(str::parse::<u64>)
            .sum::<u64>()
    );
}