// 덩치

use std::fmt::{Write, Display, Formatter, Error};
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().map(|i| i.parse::<u8>().unwrap());

    let n = input.next().unwrap() as usize;
    let mut wr = WeightRanking::new();

    for _ in 0..n {
        wr.push(input.next().unwrap(), input.next().unwrap());
    }

    println!("{}", wr);
}

struct Human {
    height: u8,
    weight: u8,
}

struct WeightRanking(Vec<Human>);

impl WeightRanking {
    #[inline]
    fn new() -> Self {
        Self(Vec::new())
    }

    #[inline]
    fn push(&mut self, height: u8, weight: u8) {
        self.0.push( Human {
            height,
            weight,
        });
    }
}

impl Display for WeightRanking {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut ranking = String::new();
        for i in 0..self.0.len() {
            let mut rank = 1_u8;
            for j in 0..self.0.len() {
                if
                    self.0[i].height < self.0[j].height &&
                    self.0[i].weight < self.0[j].weight
                {
                    rank += 1;
                }
            }
            write!(ranking, "{} ", rank).unwrap();
        }
        write!(f, "{}", ranking)
    }
}
