use std::io::{self, Read};

mod cses;
use crate::cses::introductory::trailing_zero::trailing_zero;

fn main() {
    let value = trailing_zero();
    println!("value: {}", value);
}
