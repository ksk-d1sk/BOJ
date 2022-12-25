// 마인크래프트

macro_rules! input_line {
    () => ({
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_ascii_whitespace()
            .map(|e| e.parse().unwrap())
            .collect()
    });
    ($($t: ty), +) => ({
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_ascii_whitespace();
        ($(iter.next().unwrap().parse::<$t>().unwrap()), +)
    });
}

fn main() {
    let (n, _m, b) = input_line!(u16, u16, usize);
    let mut minecraft: Vec<Vec<u16>> = Vec::new();

    for _ in 0..n {
        minecraft.push(input_line!());
    }

    let result = solution(minecraft, b);
    println!("{} {}", result.0, result.1);
}

fn solution(mut minecraft: Vec<Vec<u16>>, mut b: usize) -> (usize, u16) {
    let mut time = 0;
    let mut min = get_min(&minecraft);
    let mut max = get_max(&minecraft);

    while min != max {
        let min_count = count(&minecraft, min);
        let max_count = count(&minecraft, max);
        if b >= min_count && min_count <= max_count * 2 {
            edit(&mut minecraft, min, min + 1);
            time += min_count;
            b -= min_count;
            min += 1;
        } else {
            edit(&mut minecraft, max, max - 1);
            time += max_count * 2;
            b += max_count;
            max -= 1;
        }
    }

    (time, min)
}

fn edit(v: &mut Vec<Vec<u16>>, b: u16, a: u16) {
    for i in v {
        for j in i {
            if *j == b {
                *j = a;
            }
        }
    }
}

#[inline]
fn count(v: &Vec<Vec<u16>>, x: u16) -> usize {
    v.iter()
        .map(|i| i.iter().filter(|j| **j == x).count())
        .sum()
}

#[inline]
fn get_min(v: &Vec<Vec<u16>>) -> u16 {
    *v.iter()
        .map(|i| i.iter().min().unwrap())
        .min()
        .unwrap()
}

#[inline]
fn get_max(v: &Vec<Vec<u16>>) -> u16 {
    *v.iter()
        .map(|i| i.iter().max().unwrap())
        .max()
        .unwrap()
}
