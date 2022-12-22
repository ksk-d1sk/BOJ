// 누울 자리를 찾아라

macro_rules! input_line {
    () => ({
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().chars().map(|i| i != 'X').collect()
    });
    ($($t: ty), +) => ({
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_ascii_whitespace();
        ($(iter.next().unwrap().parse::<$t>().unwrap()), +)
    });
}

fn main() {
    let n = input_line!(u8);
    let mut room: Vec<Vec<bool>> = Vec::new();

    for _ in 0..n {
        room.push(input_line!());
    }

    println!("{} {}", hor_count(&room), ver_count(&room));
}

fn hor_count(room: &Vec<Vec<bool>>) -> u32 {
    let mut answer = 0;

    for i in 0..(room.len()) {
        let mut check = true;
        for j in 0..(room.len() - 1) {
            if room[i][j] && room[i][j+1] && check {
                answer += 1;
                check = false;
            } else if !room[i][j] {
                check = true;
            }
        }
    }

    answer
}

fn ver_count(room: &Vec<Vec<bool>>) -> u32 {
    let mut answer = 0;

    for i in 0..(room.len()) {
        let mut check = true;
        for j in 0..(room.len() - 1) {
            if room[j][i] && room[j+1][i] && check {
                answer += 1;
                check = false;
            } else if !room[j][i] {
                check = true;
            }
        }
    }

    answer
}
