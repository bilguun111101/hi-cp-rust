use std::io::{self, Read};

pub fn repetitions() -> usize {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();

    let mut left = 0;
    let mut right = 1;
    let mut longest = 0;

    while right != input.len() {
        if input[left] == input[right] {
            right += 1;
            continue;
        }
        longest = longest.max(right - left);
        left = right;
        right = left + 1;
    }

    print!("{}\n", longest);

    longest.max(right - left)
}

#[cfg(test)]
mod tests {
    use crate::cses::introductory::repetitions::repetitions;

    #[test]
    fn test_repetitions() {
        let value = repetitions();
        assert_eq!(value, 3);
    }
}
