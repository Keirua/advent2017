extern crate d6;
use d6::*;

fn main() {
	let v1 = vec![10,3,15,10,5,15,5,15,9,2,5,8,5,2,3,6];
    println!("{:?}", find_nb_iterations(v1));
}