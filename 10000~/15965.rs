// K번째 소수
use std::io;

fn main() {
    let k: usize = input();
    
    let mut prime_array = Vec::new();

    let mut x: usize = 2;
    while prime_array.len() <= k {
        let array_len = prime_array.len();
        let mut is_prime = true;
        let f_x = x as f64;

        for i in 0..array_len {
            if f_x.sqrt() < prime_array[i] as f64 {
                break;
            }

            if x % prime_array[i] == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            prime_array.push(x);
        }

        x += 1;
    }

    println!("{}", prime_array[k-1]);
}

fn input() -> usize {
    let mut n = String::new();

    io::stdin().read_line(&mut n)
        .expect("Failed to read line");

    let n: usize = n.trim().parse()
        .expect("Failed to convert to integer type");

    n
}
