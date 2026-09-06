mod cses;
// use crate::cses::introductory::weird::weird_algorithm;
// use crate::cses::introductory::missing_number::missing_number;
// use crate::cses::introductory::repetitions::repetitions;
use crate::cses::introductory::increasing_array::increasing_array;

fn main() {
    // let value = missing_number();
    // let value = repetitions();
    let value = increasing_array();

    println!("{:?}", value);
}
