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
    let (m, n, h) = input_line!(usize, usize, usize);
    let mut storage = Vec::new();

    for _ in 0..h {
        let mut floor: Vec<Vec<i8>> = Vec::new();
        for _ in 0..n {
            floor.push(input_line!());
        }
        storage.push(floor);
    }

    let result = solution(storage, n, m, h);
    println!("{result}");
}

fn solution(mut storage: Vec<Vec<Vec<i8>>>, n: usize, m: usize, h: usize) -> i32 {
    let mut answer = 0;
    let mut queue = VecDeque::new();
    const ROUTE: [(i8, i8, i8); 6] = [
        (1, 0, 0), (-1, 0, 0),
        (0, 1, 0), (0, -1, 0),
        (0, 0, 1), (0, 0, -1),
    ];

    if let Some(v) = find_value_location(&storage, 1) {
        for lc in v {
            queue.push_back(lc);
        }
    }

    while let Some((y, x, z, count)) = queue.pop_front() {
        for (dy, dx, dz) in ROUTE {
            let ny = y as i8 + dy;
            let nx = x as i8 + dx;
            let nz = z as i8 + dz;
            if 0 <= ny && ny < h as i8 &&
               0 <= nx && nx < n as i8 &&
               0 <= nz && nz < m as i8
            {
                let ny = ny as usize;
                let nx = nx as usize;
                let nz = nz as usize;
                if storage[ny][nx][nz] == 0 {
                    storage[ny][nx][nz] = 1;
                    answer = count;
                    queue.push_back((ny, nx, nz, count + 1));
                }
            }
        }
    }

    if let Some(_) = find_value_location(&storage, 0) {
        answer = -1;
    }

    answer
}

fn find_value_location(storage: &Vec<Vec<Vec<i8>>>, x: i8) -> Option<Vec<(usize, usize, usize, i32)>> {
    let mut queue = Vec::new();
    for i in 0..storage.len() {
        for j in 0..storage[i].len() {
            for k in 0..storage[i][j].len() {
                if storage[i][j][k] == x {
                    queue.push((i, j, k, 1));
                }
            }
        }
    }

    if queue.len() != 0 {
        Some(queue)
    } else {
        None
    }
}
