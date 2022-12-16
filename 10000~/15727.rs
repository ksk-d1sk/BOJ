// 조별과제를 하려는데 조장이 사라졌다

macro_rules! input_line {
    ($($t: ty), +) => ({
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_ascii_whitespace();
        ($(iter.next().unwrap().parse::<$t>().unwrap()), +)
    });
}

fn main() {
    let n = input_line!(u32);
    println!("{}", (n + 4) / 5);
}
