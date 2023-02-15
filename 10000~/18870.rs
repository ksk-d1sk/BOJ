// 좌표 압축

use std::io::{self, BufWriter, Write, Read};
use std::collections::{HashMap, BTreeSet};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut output = BufWriter::new(stdout.lock());

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();

    let mut map = HashMap::new();
    let v: Vec<i32> = input
        .split_ascii_whitespace()
        .skip(1)
        .map(|e| e.parse().unwrap())
        .collect();

    for (i, &&e) in v.iter().collect::<BTreeSet<_>>().iter().enumerate() {
        map.insert(e, i);
    }

    for e in v.iter() {
        write!(output, "{} ", map.get(e).unwrap()).unwrap();
    }
}
