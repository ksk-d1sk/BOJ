// 소수 찾기

macro_rules! input_line {
    () => ({
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_ascii_whitespace().map(|i| i.parse().unwrap()).collect()
    });
    ($($t: ty), +) => ({
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_ascii_whitespace();
        ($(iter.next().unwrap().parse::<$t>().unwrap()), +)
    });
}

fn main() {
    let _n = input_line!(u8);
    let v: Vec<usize> = input_line!();
    let prime_list = find_prime_num(1000);
    let mut count = 0;

    for i in v {
        if prime_list[i] {
            count += 1;
        }
    }

    println!("{}", count);
}

fn find_prime_num(x: usize) -> Vec<bool> {
    let mut prime_list = vec![true; x + 1];
    prime_list[0] = false;
    prime_list[1] = false;
    (2..=x).for_each(|i| {
        if prime_list[i] {
            (i..=(x / i)).for_each(|j| {
                prime_list[i * j] = false;
            })
        }
    });

    prime_list
}
