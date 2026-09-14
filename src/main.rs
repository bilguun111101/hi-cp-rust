use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let word = input.trim();
    let mut counts = [0usize; 26];

    for b in word.bytes() {
        counts[(b - b'A') as usize] += 1;
    }

    let odd_number = counts.iter().filter(|&&count| count % 2 == 1).count();
    if odd_number > 1 {
        print!("NO SOLUTION");
        return;
    }

    let mut left = String::new();
    let mut middle = String::new();

    for i in 0..26 {
        let ch = (b'A' + i as u8) as char;
        let count = counts[i];

        if count % 2 == 1 {
            middle.push(ch);
        }
        for _ in 0..count / 2 {
            left.push(ch);
        }
    }
    let right: String = left.chars().rev().collect();
    println!("{left}{middle}{right}");
}
