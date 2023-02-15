// 나는야 포켓몬 마스터 이다솜

use std::io::{self, Write, BufWriter, Read};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut output = BufWriter::new(stdout.lock());

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! input {
        () => { (input.next().unwrap()) };
        ( $t:ty ) => { (input.next().unwrap().parse::<$t>().unwrap()) };
    }

    let n = input!(usize);
    input!();

    let mut v = Vec::new();
    let mut map = HashMap::new();

    for i in 1..=n {
        let pokemon = input!();
        v.push(pokemon);
        map.insert(pokemon, i);
    }

    for i in input {
        match i.parse::<usize>() {
            Ok(idx) => writeln!(output, "{}", v[idx - 1]).unwrap(),
            Err(_)  => writeln!(output, "{}", map.get(i).unwrap()).unwrap(),
        }
    }
}
