// 미션 도네이션

use std::io;

fn main() {
    let n = input();

    for _ in 0..n {
        println!("{}", donate());
    }
}

fn donate() -> u64 {
    let m = input();
    let mut revenue = 0;
    let mut mission = Vec::new();

    for _ in 0..m {
        mission.push(line_input());
    }

    let kda = line_input();

    for mi in mission {
        let mut mission_money = 0;

        for i in 0..3 {
            let temp = mi[i] * kda[i];

            mission_money += if i == 1 {
                -temp
            } else {
                temp
            }
        }

        if 0 < mission_money {
            revenue += mission_money;
        }
    }

    revenue as u64
}

fn input() -> usize {
    let mut data = String::new();

    io::stdin().read_line(&mut data).unwrap();

    data.trim().parse().unwrap()
}

fn line_input() -> Vec<i64> {
    let mut datas = String::new();

    io::stdin().read_line(&mut datas).unwrap();

    datas.split_whitespace().map(|data| {
        data.parse().unwrap()
    }).collect()
}
