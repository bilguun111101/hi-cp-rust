use std::io::{self, Read};

pub fn coin_piles() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n = it.next().unwrap().parse::<i64>().unwrap();

    for _ in 0..n {
        let a = it.next().unwrap().parse::<i64>().unwrap();
        let b = it.next().unwrap().parse::<i64>().unwrap();

        if (a + b) % 3 == 0 && (a.max(b) <= 2 * a.min(b)) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_piles() {
        coin_piles();
        assert_eq!(0, 0);
    }
}
