use std::io::Read;

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    input.split_ascii_whitespace()
        .skip(1)
        .flat_map(str::parse::<u8>)
        .for_each(|k| {
            output.push_str(&(0..k).map(|_| '=').collect::<String>());
            output.push('\n');
        });

    print!("{output}");
}