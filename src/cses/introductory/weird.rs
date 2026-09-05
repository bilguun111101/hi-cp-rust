use std::io::{self, Read};

pub fn weird_algorithm() -> Vec<i64> {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut num: i64 = input.trim().parse().unwrap();

    let mut nums: Vec<i64> = Vec::new();

    while num != 1 {
        if num % 2 == 0 {
            num /= 2;
        } else {
            num = num * 3 + 1;
        }
        nums.push(num);
    }

    return nums;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weird_algorithm() {
        let nums = weird_algorithm();
        assert_eq!(vec![10, 5, 16, 8, 4, 2, 1], nums);
    }
}
