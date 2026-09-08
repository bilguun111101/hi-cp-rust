use std::cmp::max;
use std::io::{self, Read};

pub fn number_spirel() -> Vec<i64> {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let t: i64 = it.next().unwrap().parse().unwrap();

    let mut nums: Vec<i64> = Vec::new();

    for _ in 0..t {
        let y: i64 = it.next().unwrap().parse::<i64>().unwrap();
        let x: i64 = it.next().unwrap().parse::<i64>().unwrap();

        let n = max(y, x);

        let ans = if x > y {
            if n % 2 == 0 {
                (n - 1) * (n - 1) + y
            } else {
                n * n - y + 1
            }
        } else {
            if n % 2 == 0 {
                (n - 1) * (n - 1) + y + (n - x)
            } else {
                n * n - (y - 1) - (n - x)
            }
        };

        nums.push(ans);
    }

    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_spirel() {
        let value = number_spirel();

        assert_eq!(value, vec![8, 1, 15]);
    }
}
