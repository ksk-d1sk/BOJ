// 금민수의 개수

macro_rules! input_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap() ),+)
})}

fn main() {
    let (a, b) = input_line!(u32, u32);
    
    println!("{}", solution(a, b));
}

fn solution(a: u32, b: u32) -> usize {
    let mut geummin_numbers = Vec::from([4, 7]);

    for i in 2..=9 {
        fill_geummin_num(&mut geummin_numbers, i);
    }

    geummin_numbers.into_iter()
        .filter(|&i| a <= i && i <= b)
        .count()
}

fn fill_geummin_num(v: &mut Vec<u32>, digit: u32) {
    let limit = 2_usize.pow(digit - 1);
    let v_len = v.len();

    for i in 1..=limit {
        v.push(v[v_len - i] * 10 + 4);
        v.push(v[v_len - i] * 10 + 7);
    }
}
