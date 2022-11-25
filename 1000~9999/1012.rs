// 유기농 배추

use std::collections::VecDeque;
use std::io;
use std::fmt::Write;

fn main() {
    let mut output = String::new();
    let t = input();

    for _ in 0..t {
        let (m, n, k) = line_input();
        let mut farm = vec![vec![false; n]; m];

        for _ in 0..k {
            let (x, y) = coordinate_input();
            farm[x][y] = true;
        }

        writeln!(&mut output, "{}", solution(m, n, farm)).unwrap();
    }

    println!("{}", output);
}

fn solution(m: usize, n: usize, mut farm: Vec<Vec<bool>>) -> u16 {
    let mut count = 0;
    let mut queue = VecDeque::new();

    const MX: [i16; 4] = [-1, 0, 1,  0];
    const MY: [i16; 4] = [ 0, 1, 0, -1];

    for x in 0..m {
        for y in 0..n {
            if farm[x][y] {
                queue.push_back((x, y));
                farm[x][y] = false;
                count += 1;
            }

            while !queue.is_empty() {
                let (dx, dy) = queue.pop_front().unwrap();

                for i in 0..4 {
                    let cx = dx as i16 + MX[i];
                    let cy = dy as i16 + MY[i];

                    if
                        0    <=    cx && cx    <    m as i16
                                      &&
                        0    <=    cy && cy    <    n as i16
                    {
                        let cx = cx as usize;
                        let cy = cy as usize;

                        if farm[cx][cy] {
                            queue.push_back((cx, cy));
                            farm[cx][cy] = false;
                        }
                    }
                }
            }
        }
    }

    count
}

fn input() -> u16 {
    let mut data = String::new();

    io::stdin().read_line(&mut data).unwrap();

    data.trim().parse().unwrap()
}

fn line_input() -> (usize, usize, u16) {
    let mut datas = String::new();

    io::stdin().read_line(&mut datas).unwrap();

    let mut iter = datas.split_whitespace().map(|data| data.parse().unwrap());

    (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap() as u16)
}

fn coordinate_input() -> (usize, usize) {
    let mut datas = String::new();

    io::stdin().read_line(&mut datas).unwrap();

    let mut iter = datas.split_whitespace().map(|data| data.parse().unwrap());

    (iter.next().unwrap(), iter.next().unwrap())
}
