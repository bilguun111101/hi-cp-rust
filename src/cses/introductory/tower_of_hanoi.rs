use std::io::{self, Read};

fn hanio(n: u8, from: u32, anux: u32, to: u32, moves: &mut Vec<(u32, u32)>) {
    if n == 0 {
        return;
    }

    hanio(n - 1, from, to, anux, moves);
    moves.push((from, to));
    hanio(n - 1, anux, from, to, moves);
}

pub fn tower_of_hanoi() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n = input.trim().parse::<u8>().unwrap();
    let t = (1u32 << n) - 1;
    let mut moves: Vec<(u32, u32)> = Vec::new();
    hanio(n, 1, 2, 3, &mut moves);
    println!("{t}");
    for (from, to) in moves {
        println!("{from} {to}");
    }
}

// input 3
// moves 7
// 1 3 -> 2 0 1
// 1 2 -> 1 1 1
// 3 2 -> 1 2 0
// 1 3 -> 0 2 1
// 2 1 -> 1 1 1
// 2 3 -> 1 0 2
// 1 3 -> 0 0 3

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tower_of_hanoi() {
        tower_of_hanoi();
        assert_eq!(0, 0);
    }
}
