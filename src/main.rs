mod cses;
// use crate::cses::introductory::weird::weird_algorithm;
// use crate::cses::introductory::missing_number::missing_number;
// use crate::cses::introductory::repetitions::repetitions;
// use crate::cses::introductory::increasing_array::increasing_array;
// use crate::cses::introductory::permutations::permutations;
use crate::cses::introductory::number_spirel::number_spirel;

use std::cmp::max;
use std::io::{self, Read};

fn main() {
    // let nums = permutations();
    let nums = number_spirel();
    println!("nums: {:?}\n", nums);
}
