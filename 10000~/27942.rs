// :danceplant:

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    macro_rules! get_line {
        ( $t:ty ) => {
            input.next()
                .unwrap()
                .split_ascii_whitespace()
                .flat_map(str::parse)
                .collect::<Vec<$t>>()
        };
    }

    let mut v = Vec::new();
    let n = get_line!(usize)[0];

    for _ in 0..n {
        v.push(get_line!(i32));
    }

    let result = solve(n, v);
    print!("{}\n{}", result.0, result.1);
}

fn solve(n: usize, v: Vec<Vec<i32>>) -> (i32, String) {
    let mut ans_sum = 0;
    let mut ans_str = String::new();

    let mut gaji = (
        (n / 2 - 1, n / 2 - 1),        // (上, 左)
        (n / 2    , n / 2    ),        // (下, 右)
    );

    loop {
        let mut max = 0;
        let mut ch = 'X';

        let udlr = [
            (u(&v,    &mut gaji), 'U'),
            (d(&v, n, &mut gaji), 'D'),
            (l(&v,    &mut gaji), 'L'),
            (r(&v, n, &mut gaji), 'R'),
        ];

        for (nour, c) in udlr {
            if max < nour {
                max = nour;
                ch = c;
            }
        }

        match ch {
            'U' => gaji.0.0 -= 1,
            'D' => gaji.1.0 += 1,
            'L' => gaji.0.1 -= 1,
            'R' => gaji.1.1 += 1,
            _   => break,
        }

        ans_sum += max;
        ans_str.push(ch);
    }

    (ans_sum, ans_str)
}

// 가지 윗부분 양분
fn u(
    v: &Vec<Vec<i32>>,
    gaji: &mut ((usize, usize),(usize, usize))
) -> i32 {
    let mut sum = 0;

    if gaji.0.0 != 0 {
        for i in 0..=(gaji.1.1 - gaji.0.1) {
            sum += v[gaji.0.0 - 1][gaji.0.1 + i];
        }
    }

    sum
}

// 가지 아랫부분 양분
fn d(
    v: &Vec<Vec<i32>>,
    n: usize,
    gaji: &mut ((usize, usize),(usize, usize))
) -> i32 {
    let mut sum = 0;

    if gaji.1.0 != n - 1 {
        for i in 0..=(gaji.1.1 - gaji.0.1) {
            sum += v[gaji.1.0 + 1][gaji.0.1 + i];
        }
    }

    sum
}

// 가지 왼쪽부분 양분
fn l(
    v: &Vec<Vec<i32>>,
    gaji: &mut ((usize, usize),(usize, usize))
) -> i32 {
    let mut sum = 0;

    if gaji.0.1 != 0 {
        for i in 0..=(gaji.1.0 - gaji.0.0) {
            sum += v[gaji.0.0 + i][gaji.0.1 - 1];
        }
    }

    sum
}


// 가지 오른쪽부분 양분
fn r(
    v: &Vec<Vec<i32>>,
    n: usize,
    gaji: &mut ((usize, usize),(usize, usize))
) -> i32 {
    let mut sum = 0;

    if gaji.1.1 != n - 1 {
        for i in 0..=(gaji.1.0 - gaji.0.0) {
            sum += v[gaji.0.0 + i][gaji.1.1 + 1];
        }
    }

    sum
}