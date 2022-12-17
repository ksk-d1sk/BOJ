// 불!

use std::io::{self, Read};
use std::collections::VecDeque;
use self::Object::*;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let r: usize = input.next().unwrap().parse().unwrap();
    let c: usize = input.next().unwrap().parse().unwrap();
    let mut maze = Vec::new();

    for _ in 0..r {
        let v = input.next().unwrap().chars().collect();
        maze.push(v);
    }

    let result = solution(maze, r, c);
    match result {
        Ok(x)  => println!("{}", x),
        Err(e) => println!("{}", e),
    }
}

fn solution(mut maze: Vec<Vec<char>>, row: usize, col: usize) -> Result<u32, &'static str> {
    let mut answer = Err("IMPOSSIBLE");
    let mut queue = VecDeque::new();
    const ROUTE: [(i16, i16); 4] = [
        (1, 0), (-1, 0),
        (0, 1), (0, -1),
    ];

    // 큐에 'F'와'J' 위치를 push
    queue_init(&maze, &mut queue);

    while let Some((obj, x, y)) = queue.pop_front() {
        // J가 테두리쪽에 있다면 종료
        if let Human(count) = obj {
            if x == 0 || x == row - 1 ||
               y == 0 || y == col - 1
            {
                answer = Ok(count);
                break;
            }
        }

        for (dx, dy) in ROUTE {
            let nx = dx + x as i16;
            let ny = dy + y as i16;
            if 0 <= nx && nx < row as i16 &&
               0 <= ny && ny < col as i16
            {
                let nx = nx as usize;
                let ny = ny as usize;
                match obj {
                    Human(count) => {
                        if maze[nx][ny] != '#' &&
                           maze[nx][ny] != 'F' &&
                           maze[nx][ny] != 'J'
                        {
                            maze[nx][ny] = 'J';
                            queue.push_back((Human(count + 1), nx, ny));
                        }
                    },
                    Fire  => {
                        if maze[nx][ny] != '#' &&
                           maze[nx][ny] != 'F'
                        {
                            maze[nx][ny] = 'F';
                            queue.push_back((Fire, nx, ny));
                        }
                    },
                }
            }
        }
    }

    answer
}

enum Object {
    Human(u32),
    Fire,
}

fn queue_init(maze: &Vec<Vec<char>>, queue: &mut VecDeque<(Object, usize, usize)>) {
    if let Some(fire) = find_value_location(&maze, 'F') {
        for (x, y) in fire {
            queue.push_back((Fire, x, y));
        }
    }
    for (x, y) in find_value_location(&maze, 'J').unwrap() {
        queue.push_back((Human(1), x, y));
    }
}

fn find_value_location(maze: &Vec<Vec<char>>, value: char) -> Option<Vec<(usize, usize)>> {
    let mut location = Vec::new();
    for x in 0..maze.len() {
        for y in 0..maze[x].len() {
            if maze[x][y] == value {
                location.push((x, y));
            }
        }
    }

    if location.len() != 0 {
        Some(location)
    } else {
        None
    }
}
