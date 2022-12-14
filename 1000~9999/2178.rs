// 미로 탐색

use std::collections::VecDeque;

macro_rules! input_line {
    () => ({
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim()
            .chars()
            .map(|i| i != '0')
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
    let (n, m) = input_line!(usize, usize);
    let mut maze: Vec<Vec<bool>> = Vec::new();

    for _ in 0..n {
        maze.push(input_line!());
    }

    let result = solution(maze, n, m);
    println!("{result}");
}

fn solution(maze: Vec<Vec<bool>>, n: usize, m: usize) -> u16 {
    let mut board = vec![vec![0; m]; n];
    let mut queue = VecDeque::new();
    const ROUTE: [(i8, i8); 4] = [
        (1, 0), (-1, 0),
        (0, 1), (0, -1),
    ];

    queue.push_back((0, 0, 1));

    while let Some((x, y, count)) = queue.pop_front() {
        board[x][y] = count;

        for (dx, dy) in ROUTE {
            let nx = x as i8 + dx;
            let ny = y as i8 + dy;
            if 0 <= nx && nx < n as i8 &&
               0 <= ny && ny < m as i8
            {
                let nx = nx as usize;
                let ny = ny as usize;
                if maze[nx][ny] && board[nx][ny] == 0 {
                    board[nx][ny] = count + 1;
                    queue.push_back((nx, ny, count + 1));
                }
            }
        }
    }

    board[n-1][m-1]
}
