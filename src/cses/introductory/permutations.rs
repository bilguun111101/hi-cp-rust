use std::io::{self, Read};

pub fn permutations() -> Vec<i64> {
    let mut value = String::new();
    io::stdin().read_to_string(&mut value).unwrap();
    let mut it = value.split_whitespace();
    let n: i64 = it.next().unwrap().parse().unwrap();

    if n == 2 || n == 3 {
        print!("NO SOLUTION");
        return vec![];
    }

    let mut nums: Vec<i64> = Vec::new();
    for i in (2..n + 1).step_by(2) {
        nums.push(i);
    }
    for i in (1..n + 1).step_by(2) {
        nums.push(i);
    }

    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations() {
        let nums = permutations();
        assert_eq!(vec![2, 4, 6, 8, 10, 1, 3, 5, 7, 9, 11], nums);
    }
}
