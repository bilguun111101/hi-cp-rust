mod cses;
use crate::cses::introductory::weird::weird_algorithm;

fn main() {
    let nums = weird_algorithm();

    println!("{:?}", nums);
}
