// 배열 합치기

use std::io::Read;
use std::fmt::Write;

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    macro_rules! get_line {
        () => { input.next().unwrap().split_ascii_whitespace().flat_map(str::parse::<i32>).collect::<Vec<_>>() };
        ( $($t:ty),+ ) => {{
            let mut iter = input.next().unwrap().split_ascii_whitespace();
            ( $(iter.next().unwrap().parse::<$t>().unwrap()),+ )
        }}
    }

    let (n, m) = get_line!(usize, usize);
    let arr_a = get_line!();
    let arr_b = get_line!();
    let mut answer_arr = Vec::with_capacity(n + m);

    let mut aidx = 0;
    let mut bidx = 0;

    for _ in 0..(n + m) {
        if bidx == m {
            answer_arr.push(arr_a[aidx]);
            aidx += 1;
        } else if aidx == n {
            answer_arr.push(arr_b[bidx]);
            bidx += 1;
        } else if arr_a[aidx] <= arr_b[bidx] {
            answer_arr.push(arr_a[aidx]);
            aidx += 1;
        } else {
            answer_arr.push(arr_b[bidx]);
            bidx += 1;
        }
    }

    for elem in answer_arr {
        write!(output, "{elem} ").unwrap();
    }

    println!("{output}");
}