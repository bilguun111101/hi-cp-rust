use std::io::{self, Read};

pub fn trailing_zero() -> u64 {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n = input.trim().parse::<u64>().unwrap();
    let mut f: u64 = 5;
    let mut result: u64 = 0;

    while n % f == 0 {
        let rem = n / f;
        result += rem;
        f *= 5;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trailing_zero() {
        let value = trailing_zero();
        assert_eq!(value, 4);
    }
}
