// Gum Gum for Jay Jay

use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    print!("{}", input.lines().count());
}
