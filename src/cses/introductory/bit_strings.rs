use std::io::{self, Read};

pub fn bit_strings() -> u64 {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: u64 = input.trim().parse().unwrap();

    let mut base: u64 = 2;
    let mut exp: u64 = n;
    let mut result: u64 = 1;
    let modulo: u64 = 1_000_000_007;

    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulo;
        }
        base = base * base % modulo;
        exp /= 2;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bit_strings() {
        let value = bit_strings();
        assert_eq!(value, 8);
    }
}
