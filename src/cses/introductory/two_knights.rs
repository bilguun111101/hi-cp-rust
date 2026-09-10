use std::io::{self, Read};

pub fn two_knights() -> Vec<i64> {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n = it.next().unwrap().parse::<i64>().unwrap();

    let mut nums: Vec<i64> = Vec::new();

    for i in 1..n + 1 {
        let board_size = i * i;
        let total_board_rectangles = (board_size * (board_size - 1)) / 2;
        let total_rectangles = (i - 1) * (i - 2) * 2;
        let total_rectangles_with_pair = total_rectangles * 2;

        nums.push(total_board_rectangles - total_rectangles_with_pair);
    }

    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_knights() {
        let value = two_knights();
        assert_eq!(value, vec![0, 6, 28, 96, 252, 550, 1056, 1848]);
    }
}
