// 종이의 개수

use std::io::{self, Read};
use std::fs::File;

fn main() {
    let mut input = String::new();
    // io::stdin().read_to_string(&mut input).unwrap();
    let mut f = File::open("input.txt").unwrap();
    f.read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let n: usize = input.next().unwrap().parse().unwrap();
    let mut paper: Vec<Vec<i8>> = Vec::new();

    for _ in 0..n {
        paper.push(input.next().unwrap().split(' ').map(|i| i.parse::<i8>().unwrap()).collect());
    }

    let result = solution(&paper, n, 0, 0);
    println!("{}\n{}\n{}", result[0], result[1], result[2]);
}

fn solution(paper: &Vec<Vec<i8>>, n: usize, x: usize, y: usize) -> [u32; 3] {
    let compare = paper[x][y];
    let check = paper.iter()
        .skip(x)
        .take(n)
        .map(|s| s.iter().skip(y).take(n))
        .all(|iter| {
            let mut check = true;
            for &i in iter {
                if i != compare {
                    check = false;
                    break;
                }
            }
            check
        });

    if check {
        match compare {
            -1 => [1, 0, 0],
            0  => [0, 1, 0],
            1  => [0, 0, 1],
            _  => panic!(),
        }
    } else {
        let mut answer = [0, 0, 0];
        let slice = n / 3;
        for v in [solution(paper, slice, x, y),
                  solution(paper, slice, x, y + slice),
                  solution(paper, slice, x, y + slice * 2),
                  solution(paper, slice, x + slice, y),
                  solution(paper, slice, x + slice, y + slice),
                  solution(paper, slice, x + slice, y + slice * 2),
                  solution(paper, slice, x + slice * 2, y),
                  solution(paper, slice, x + slice * 2, y + slice),
                  solution(paper, slice, x + slice * 2, y + slice * 2)]
        {
            for i in 0..3 {
                answer[i] += v[i];
            }
        }

        answer
    }
}
