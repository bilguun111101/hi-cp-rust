use std::io::{self, Read};

pub fn increasing_array() -> i64 {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let mut nums: Vec<i64> = it.map(|x| x.parse::<i64>().unwrap()).collect();
    let mut val = 0;

    for i in 1..n {
        let minus = nums[i - 1] - nums[i];
        if minus < 0 {
            continue;
        }
        nums[i] = nums[i] + minus;
        val = val + minus;
    }

    val
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_increasing_array() {
        let value = increasing_array();
        assert_eq!(value, 8999999991);
    }
}
