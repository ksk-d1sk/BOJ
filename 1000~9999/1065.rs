// 한수

fn main() {
    let n = uint_input();
    let mut count = 0;

    for i in 1..=n {
        let mut check = true;

        if 100 <= i {
            check = is_hansu(i);
        }

        if check {
            count += 1;
        }
    }

    println!("{}", count);
}

fn is_hansu(n: u32) -> bool {
    let mut digit = 0;
    let mut n_vec = Vec::new();

    {
        let mut cp_n = n;

        while cp_n != 0 {
            n_vec.push(cp_n as i64 % 10);
            cp_n /= 10;
            digit += 1;
        }
    }

    let mut arr = Vec::new();
    for i in 0..(digit - 1) {
        arr.push(n_vec[i] - n_vec[i+1]);
    }

    is_equals(arr)
}

fn is_equals<T>(vec: Vec<T>) -> bool
where
    T: std::cmp::PartialEq,
{
    assert!(vec.len() > 1);

    let temp = &vec[0];
    let mut check = true;

    for i in 1..vec.len() {
        if temp != &vec[i] {
            check = false;
            break;
        }
    }

    check
}

fn uint_input() -> u32 {
    let mut n = String::new();

    std::io::stdin().read_line(&mut n).unwrap();

    n.trim().parse().unwrap()
}
