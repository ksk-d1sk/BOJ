// 덱

use std::io;
use std::fmt::Write;
use std::collections::VecDeque;

fn main() {
    let n = input();
    let mut output = String::new();
    let mut deque: VecDeque<i32> = VecDeque::new();

    for _ in 0..n {
        let data = line_input();
        let cmd = &data[0][..];

        match cmd {
            "push_front" => deque.push_front(data[1].parse().unwrap()),
            "push_back"  => deque.push_back (data[1].parse().unwrap()),
            "pop_front"  => writeln!(&mut output, "{}", deque.pop_front().unwrap_or(-1)).unwrap(),
            "pop_back"   => writeln!(&mut output, "{}", deque.pop_back ().unwrap_or(-1)).unwrap(),
            "size"       => writeln!(&mut output, "{}", deque.len()).unwrap(),
            "empty"      => writeln!(&mut output, "{}", if deque.is_empty() { 1 } else { 0 }).unwrap(),
            "front"      => writeln!(&mut output, "{}", deque.front().unwrap_or(&-1)).unwrap(),
            "back"       => writeln!(&mut output, "{}", deque.back().unwrap_or(&-1)).unwrap(),
            _            => (),
        }
    }

    println!("{}", output);
}

fn input() -> u16 {
    let mut data = String::new();

    io::stdin().read_line(&mut data).unwrap();

    data.trim().parse().unwrap()
}

fn line_input() -> Vec<String> {
    let mut datas = String::new();

    io::stdin().read_line(&mut datas).unwrap();

    datas.split_whitespace().map(|data| {
        data.to_string()
    }).collect()
}
