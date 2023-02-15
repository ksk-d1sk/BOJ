// Serca

use std::io::{self, Read, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut output = BufWriter::new(stdout);

    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! next {
        ( $t:ty ) => { input.next().unwrap().parse::<$t>().unwrap() };
    }

    for _ in 0..next!(usize) {
        writeln!(
            output,
" @@@   @@@ 
@   @ @   @
@    @    @
@         @
 @       @ 
  @     @  
   @   @   
    @ @    
     @     "
        ).unwrap();
    }
}
