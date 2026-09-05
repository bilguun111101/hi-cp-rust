use std::io::{self, Read};

pub fn missing_number() -> i64 {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();

    let addition: i64 = it.map(|x| x.parse::<i64>().unwrap()).sum();
    let total = n * (n + 1) / 2;

    return total - addition;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_number() {
        let value = missing_number();
        assert_eq!(value, 4);
    }
}
