// 부녀회장이 될테야

use std::io::{self, Read};
use std::fmt::Write;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().map(|i| i.parse::<usize>().unwrap());

    let mut output = String::new();
    let t = input.next().unwrap();

    for _ in 0..t {
        let (k, n) = (input.next().unwrap(), input.next().unwrap());
        let mut apartment = Apartment::new(n);

        apartment.fill(k);

        writeln!(output, "{}", apartment.get(k, n).unwrap_or(&0)).unwrap();
    }

    print!("{}", output);
}

struct Apartment(Vec<Vec<usize>>);

impl Apartment {
    /// n호까지 있는 아파트를 생성합니다
    fn new(n: usize) -> Self {
        let mut v = Vec::new();
        v.push(Vec::new());

        for i in 1..=n {
            v[0].push(i);
        }

        Self(v)
    }

    /// k층까지 모든 방에 사람을 채워넣습니다
    fn fill(&mut self, k: usize) {
        for floor in self.0.len()..=k {
            let mut v = Vec::new();
            self.fill_floor(&mut v, floor);
            self.0.push(v);
        }
    }

    /// 채워넣을 층을 입력받아 결과를 v파라미터에 저장합니다
    fn fill_floor(&self, v: &mut Vec<usize>, floor: usize) {
        for i in 1..=self.0[0].len() {
            let mut sum = 0;

            for j in 0..i {
                sum += self.0[floor - 1][j];
            }

            v.push(sum);
        }
    }

    /// k층 n호에 사는 인원수를 Option형태로 가져옵니다
    #[inline]
    fn get(&self, k: usize, n: usize) -> Option<&usize> {
        self.0.get(k).and_then(|floor| floor.get(n - 1))
    }
}
