mod cses;
// use crate::cses::introductory::weird::weird_algorithm;
// use crate::cses::introductory::missing_number::missing_number;
// use crate::cses::introductory::repetitions::repetitions;
// use crate::cses::introductory::increasing_array::increasing_array;
use crate::cses::introductory::permutations::permutations;

use std::io::{self, Read};

fn main() {
    // let value = missing_number();
    // let value = repetitions();
    let nums = permutations();
    // let value = increasing_array();

    // println!("{:?}", value);

    // let mut value = String::new();
    // io::stdin().read_to_string(&mut value).unwrap();
    // let mut it = value.split_whitespace();
    // let n: i64 = it.next().unwrap().parse().unwrap();

    // if n == 2 || n == 3 {
    //     print!("NO SOLUTION");
    //     return;
    // }

    // for i in (2..n + 1).step_by(2) {
    //     print!("{} ", i);
    // }
    // for i in (1..n + 1).step_by(2) {
    //     print!("{} ", i);
    // }
}
