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
    let mid = width / 2;
    let q = {
        if mid < r && mid < c { 4 }
        else if mid < r       { 3 }
        else if mid < c       { 1 }
        else                  { 2 }
    };

    // println!("{}사분면", q);

    if width == 1 {
        0
    } else {
        match q {
            2 => solution(mid, r, c),
            1 => solution(mid, r, c - mid) + fpow(mid, 2),
            3 => solution(mid, r - mid, c) + fpow(mid, 2) * 2,
            4 => solution(mid, r - mid, c - mid) + fpow(mid, 2) * 3,
            _ => panic!(),
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
