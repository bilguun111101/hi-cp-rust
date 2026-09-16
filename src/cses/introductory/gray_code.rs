use std::io::{self, Read};

pub fn gray_code() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();
    for i in 0..1u64 << n {
        println!("{:0width$b}", i ^ (i >> 1), width = n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gray_code() {
        gray_code();
        assert_eq!(0, 0);
    }
}
