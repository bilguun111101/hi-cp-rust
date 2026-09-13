use std::io::{self, Read};

pub fn two_set() -> (Vec<i64>, Vec<i64>) {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n = it.next().unwrap().parse::<i64>().unwrap();
    let sum = n * (n + 1) / 2;
    if sum % 2 != 0 {
        print!("NO");
        return (vec![], vec![]);
    }

    let mut a: Vec<i64> = Vec::new();
    let mut b: Vec<i64> = Vec::new();

    if n % 4 == 3 {
        a.extend([1, 2]);
        b.push(3);

        for i in (4..=n).step_by(4) {
            a.extend([i, i + 3]);
            b.extend([i + 1, i + 2]);
        }
    } else {
        for i in (1..=n).step_by(4) {
            a.extend([i, i + 3]);
            b.extend([i + 1, i + 2]);
        }
    }

    println!("a: {:?}\n", a);
    println!("b: {:?}\n", b);

    return (a, b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_set() {
        let (a, b) = two_set();
        assert_eq!(a, vec![1, 2, 4, 7, 8, 11]);
        assert_eq!(b, vec![3, 5, 6, 9, 10]);
    }
}
