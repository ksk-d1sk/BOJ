// 네 수

macro_rules! input_line {
    ($($t: ty), +) => ({
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_ascii_whitespace();
        ($(iter.next().unwrap().parse::<$t>().unwrap()), +)
    });
}

fn main() {
    let (mut a, b, mut c, d) = input_line!(String, String, String, String);
    println!(
        "{}",
        {
            a.push_str(b.as_str());
            c.push_str(d.as_str());
            a.parse::<u64>().unwrap() + c.parse::<u64>().unwrap()
        }
    )
}
