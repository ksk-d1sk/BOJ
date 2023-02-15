// N-Queen

use std::io;

struct BOJ9663 {
    n: usize,
    cnt: usize,
    check_1: Vec<bool>,
    check_2: Vec<bool>,
    check_3: Vec<bool>,
}

impl BOJ9663 {
    fn new(n: usize) -> Self {
        Self {
            n,
            cnt: 0,
            check_1: vec![true; n],
            check_2: vec![true; n * 2 - 1],
            check_3: vec![true; n * 2 - 1],
        }
    }

    fn solve(&mut self, y: usize) {
        if y == self.n {
            self.cnt += 1;
        } else {
            for x in 0..self.n {
                if self.check_1[x] && self.check_2[self.n + y - x - 1] && self.check_3[x + y] {
                    self.check_1[x] = false;
                    self.check_2[self.n + y - x - 1] = false;
                    self.check_3[x + y] = false;
                    self.solve(y + 1);
                    self.check_1[x] = true;
                    self.check_2[self.n + y - x - 1] = true;
                    self.check_3[x + y] = true;
                }
            }
        }
    }

    fn get_result(&self) -> usize {
        self.cnt
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse().unwrap();

    let mut boj_9663 = BOJ9663::new(n);
    boj_9663.solve(0);

    println!("{}", boj_9663.get_result());
}
