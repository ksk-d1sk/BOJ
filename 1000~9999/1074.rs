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
    let q = {
        let mid = width / 2;
        if mid < r && mid < c { 4 }
        else if mid < r       { 3 }
        else if mid < c       { 1 }
        else                  { 2 }
    };

    // println!("{}사분면", q);

    if width == 2 {
        match q {
            2 => 0,
            1 => 1,
            3 => 2,
            4 => 3,
            _ => panic!(),
        }
    } else {
        match q {
            2 => solution(width / 2, r, c),
            1 => solution(width / 2, r, c - width / 2) + fpow(width / 2, 2),
            3 => solution(width / 2, r - width / 2, c) + fpow(width / 2, 2) * 2,
            4 => solution(width / 2, r - width / 2, c - width / 2) + fpow(width / 2, 2) * 3,
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
