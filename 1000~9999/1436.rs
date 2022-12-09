// 영화감독 숌

macro_rules! input_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap() ),+)
})}

fn main() {
    let n = input_line!(u16);
    let mut count = 0;
    let mut i = 666;

    while count != n {
        if is_666(i) {
            count += 1;
        }
        i += 1;
    }

    println!("{}", i - 1);
}

fn is_666(mut i: u32) -> bool {
    let mut count = 0;
    while i != 0 && count < 3 {
        if i % 10 == 6 {
            count += 1;
        } else {
            count = 0
        }
        i /= 10;
    }

    3 <= count
}
