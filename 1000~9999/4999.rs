// 아!

macro_rules! input_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap() ),+)
})}

fn main() {
    let me = input_line!(String);
    let you = input_line!(String);
    println!(
        "{}", if me.len() < you.len() { "no" } else { "go" }
    )
}
