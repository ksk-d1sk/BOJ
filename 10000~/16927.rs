// 배열 돌리기 2

use std::io::{self, Read};

fn spin(v: &mut Vec<Vec<&str>>, n: usize, m: usize, r: usize, depth: usize) {
    if 1 < n && 1 < m {
        spin(v, n - 2, m - 2, r, depth + 1);

        let mut x = depth;
        let mut y = depth;
        let r = r % ((n - 1) * 2   + (m - 1) * 2);

        let route: [(usize, i16, i16); 4] = [
            (n,  1, 0), (m, 0,  1),
            (n, -1, 0), (m, 0, -1),
        ];

        for _ in 0..r {
            let mut temp = v[x][y];
            for (mv, dx, dy) in route {
                for _ in 1..mv {
                    x = (dx + x as i16) as usize;
                    y = (dy + y as i16) as usize;

                    (temp, v[x][y]) = (v[x][y], temp);
                }
            }
        }
    }
}

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    macro_rules! get_line {
        () => { input.next().unwrap().split_ascii_whitespace().collect() };
        ( $( $t:ty ),+ ) => {{
            let mut line = input.next().unwrap().split_ascii_whitespace();
            ($(line.next().unwrap().parse::<$t>().unwrap()), +)
        }};
    }

    let (n, m, r) = get_line!(usize, usize, usize);
    let mut v = Vec::new();

    for _ in 0..n {
        v.push(get_line!());
    }

    spin(&mut v, n, m, r, 0);

    for i in v {
        for j in i {
            output.push_str(j);
            output.push(' ');
        }
      output.push('\n');
    }

    print!("{output}")
}