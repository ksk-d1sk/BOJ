// 행렬 곱셈

use std::io::Read;
use std::fmt::Write;

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ( $(input.next().unwrap().parse::<$t>().unwrap()),+ ) };
    }

    macro_rules! get_metrix {
        ($n:expr, $m:expr) => {{
            let mut metrix = Vec::with_capacity($n);
            for _ in 0..$n {
                let mut row = Vec::with_capacity($m);
                for _ in 0..$m {
                    row.push(get!(i32));
                }
                metrix.push(row);
            }
            metrix
        }};
    }

    let (n, m) = get!(usize, usize);
    let a = get_metrix!(n, m);

    let (_, k) = get!(usize, usize);
    let b = get_metrix!(m, k);

    for row in metrix_mul(a, b) {
        for elem in row {
            write!(output, "{} ", elem).unwrap();
        }
        output.push('\n');
    }

    println!("{output}");
}

fn metrix_mul(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let row_a = a.len();
    let row_b = b.len();
    let col_b = b[0].len();

    let mut answer = vec![vec![0; col_b]; row_a];

    for i in 0..row_a {
        for j in 0..col_b {
            for k in 0..row_b {
                answer[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    answer
}