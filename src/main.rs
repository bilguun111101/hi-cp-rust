mod cses;
// use crate::cses::introductory::weird::weird_algorithm;
use crate::cses::introductory::missing_number::missing_number;

use std::io::{self, Read};

fn main() {
    // let nums = weird_algorithm();
    let value = missing_number();

    println!("{:?}", value);
}
