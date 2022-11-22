// 체스판 다시 칠하기

use std::io;
use std::cmp;

fn main() {
    let mut board = Vec::new();
    let height;
    let width;

    let temp = line_input();
    (height, width) = (temp[0], temp[1]);

    for _ in 0..height {
        board.push(input());
    }

    println!("{}", solution(height, width, board));
}

fn solution(height: usize, width: usize, board: Vec<String>) -> u32 {
    let mut min = 999;

    for i in 0..=(height - 8) {
        for j in 0..=(width - 8) {
            let mut a = 1;
            let mut met_1 = 0;
            let mut met_2 = 0;

            for line in &board[i..(i + 8)] {
                let line = &line[j..(j + 8)];
                let temp = repaint_count(line, a);

                met_1 += temp.0;
                met_2 += temp.1;

                a += 1;
            }

            let repaint = cmp::min(met_1, met_2);

            if repaint < min {
                min = repaint;
            }
        }
    }

    min
}

fn repaint_count(line: &str, a: u8) -> (u32, u32) {
    assert_eq!(line.len(), 8);

    let chese_board = if a & 1 == 1 {
        "WBWBWBWB"
    } else {
        "BWBWBWBW"
    }.as_bytes();

    let line = line.as_bytes();

    let mut x = 0;
    let mut y = 0;

    for i in 0..8 {
        if chese_board[i] == line[i] {
            x += 1;
        } else {
            y += 1;
        }
    }

    (x, y)
}

fn line_input() -> Vec<usize> {
    let mut datas = String::new();

    io::stdin().read_line(&mut datas).unwrap();

    datas.split_whitespace().map(|data| {
        data.parse().unwrap()
    }).collect()
}

fn input() -> String {
    let mut data = String::new();

    io::stdin().read_line(&mut data).unwrap();

    data.trim().to_string()
}
