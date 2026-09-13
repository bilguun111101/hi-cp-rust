use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n = input.trim().parse::<u64>().unwrap();
    let mut f: u64 = 5;
    let mut result: u64 = 0;

    while f <= n {
        let r = n / f;
        result += r;
        f *= 5;
    }

    print!("{}", result);
}
