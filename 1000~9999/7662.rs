// 이중 우선순위 큐

use std::io::{self, Read};
use std::collections::BTreeMap;
use std::fmt::Write;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut output = String::new();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();
    let mut buf = buf.split_ascii_whitespace();

    macro_rules! next {
        (       ) => { buf.next().unwrap() };
        ( $t:ty ) => { buf.next().unwrap().parse::<$t>().unwrap() };
    }

    for _ in 0..next!(usize) {
        let mut map = BTreeMap::new();
        for _ in 0..next!(usize) {
            if next!() == "I" {
                map.entry(next!(i32)).and_modify(|curr| *curr += 1).or_insert(0_usize);
            } else {
                if next!(i8) == 1 {
                    if let Some((key, value)) = map.iter().rev().next().map(|(k, v)| (*k, *v)) {
                        if value < 1 {
                            map.remove(&key);
                        } else {
                            *map.get_mut(&key).unwrap() -= 1;
                        }
                    }
                } else {
                    if let Some((key, value)) = map.iter().next().map(|(k, v)| (*k, *v)) {
                        if value < 1 {
                            map.remove(&key);
                        } else {
                            *map.get_mut(&key).unwrap() -= 1;
                        }
                    }
                }
            }
        }

        if !map.is_empty() {
            writeln!(
                output,
                "{} {}",
                *map.iter().rev().next().unwrap().0,
                *map.iter().next().unwrap().0
            ).unwrap();
        } else {
            output.push_str("EMPTY\n");
        }
    }

    print!("{output}");
}
