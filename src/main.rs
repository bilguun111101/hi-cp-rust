use std::io::{self, Read};

// use crate::cses::introductory::tower_of_hanoi::tower_of_hanoi;
// mod cses;

fn hanoi(n: u8, from: u8, aux: u8, to: u8, moves: &mut Vec<(u8, u8)>) {
    if n == 0 {
        return;
    }

    hanoi(n - 1, from, to, aux, moves);
    moves.push((from, to));
    hanoi(n - 1, aux, from, to, moves);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n = input.trim().parse::<u8>().unwrap();
    let t = (1u32 << n) - 1;
    println!("{t}");
    let mut moves: Vec<(u8, u8)> = Vec::new();
    hanoi(n, 1, 2, 3, &mut moves);

    for (from, to) in moves {
        println!("{from} {to}");
    }
}
