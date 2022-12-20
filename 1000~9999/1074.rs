// Z

macro_rules! input_line {
    ($($t: ty), +) => ({
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_ascii_whitespace();
        ($(iter.next().unwrap().parse::<$t>().unwrap()), +)
    });
}

fn main() {
    let (n, r, c) = input_line!(u8, usize, usize);
    let width = fpow(2, n);
    println!("{}", solution(width, r + 1, c + 1));
}

fn solution(width: usize, r: usize, c: usize) -> usize {
    if width == 1 {
        0
    } else {
        let mid = width / 2;
        if mid < r && mid < c {
            solution(mid, r - mid, c - mid) + fpow(mid, 2) * 3
        } else if mid < r {
            solution(mid, r - mid, c) + fpow(mid, 2) * 2
        } else if mid < c {
            solution(mid, r, c - mid) + fpow(mid, 2)
        } else {
            solution(mid, r, c)
        }
    }
}

fn fpow(mut c: usize, mut n: u8) -> usize {
    let mut res = 1;
    while 0 < n {
        if n & 1 == 1 {
            res *= c;
        }
        c *= c;
        n >>= 1;
    }

    res
}
