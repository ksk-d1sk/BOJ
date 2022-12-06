// 이번학기 평점은 몇점?

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse().unwrap();

    let mut total_score  = 0.0;
    let mut total_credit = 0.0;
    
    for _ in 0..n {
        input.next();
        let credit = input.next().unwrap().parse::<f64>().unwrap();
        let score  = input.next().unwrap();
        let score  = score_converter(score);

        total_credit += credit;
        total_score  += credit * score;
    }

    let result = (total_score / total_credit * 100.0).round() / 100.0;
    println!("{:.2}", result);
}

fn score_converter(score: &str) -> f64 {
    let mut answer = 0.0;

    answer += match &score[0..1] {
        "A" => 4.0,
        "B" => 3.0,
        "C" => 2.0,
        "D" => 1.0,
        _   => 0.0,
    };

    answer += match score.get(1..2) {
        Some("+") => 0.3,
        Some("-") => -0.3,
        _   => 0.0,
    };

    answer
}
