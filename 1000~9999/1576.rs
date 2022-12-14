// 토마토

use std::collections::VecDeque;

macro_rules! input_line {
    () => ({
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_ascii_whitespace()
            .map(|i| i.parse().unwrap())
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
    let (m, n) = input_line!(usize, usize);
    let mut storage: Vec<Vec<i8>> = Vec::new();

    for _ in 0..n {
        storage.push(input_line!());
    }

    let result = solution(storage, n, m);
    println!("{result}");
}

fn solution(mut storage: Vec<Vec<i8>>, n: usize, m: usize) -> i32 {
    let mut answer = 0;
    let mut queue = VecDeque::new();
    const ROUTE: [(i16, i16); 4] = [
        (1, 0), (-1, 0),
        (0, 1), (0, -1),
    ];

    if let Some(v) = find_value_location(&storage, 1) {
        for lc in v {
            queue.push_back(lc);
        }
    }

    while let Some((x, y, count)) = queue.pop_front() {
        for (dx, dy) in ROUTE {
            let nx = x as i16 + dx;
            let ny = y as i16 + dy;
            if 0 <= nx && nx < n as i16 &&
               0 <= ny && ny < m as i16
            {
                let nx = nx as usize;
                let ny = ny as usize;
                if storage[nx][ny] == 0 {
                    storage[nx][ny] = 1;
                    answer = count;
                    queue.push_back((nx, ny, count + 1));
                }
            }
        }
    }

    if let Some(_) = find_value_location(&storage, 0) {
        answer = -1;
    }

    answer
}

fn find_value_location(storage: &Vec<Vec<i8>>, x: i8) -> Option<Vec<(usize, usize, i32)>> {
    let mut queue = Vec::new();
    for i in 0..storage.len() {
        for j in 0..storage[i].len() {
            if storage[i][j] == x {
                queue.push((i, j, 1));
            }
        }
    }

    if queue.len() != 0 {
        Some(queue)
    } else {
        None
    }
}
