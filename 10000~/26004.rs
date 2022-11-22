use std::io;

fn main() {
    let _ = input();
    let s = input();

    println!("{}", solution(s));
}

fn solution(s: String) -> i32 {
    let mut h_i_a_r_c = vec![0; 5];

    for c in s.chars() {
        h_i_a_r_c[
            match c {
                'H' => 0,
                'I' => 1,
                'A' => 2,
                'R' => 3,
                'C' => 4,
                _   => continue,
            }
        ] += 1;
    }

    *h_i_a_r_c.iter().min().unwrap()
}

fn input() -> String {
    let mut data = String::new();

    io::stdin().read_line(&mut data).unwrap();

    data.trim().to_string()
}
