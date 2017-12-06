extern crate d4;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("d4input.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
    	.expect("something went wrong reading the file");

	println!("Valid passwords: {}", d4::CountRegular::count_valid(&contents));
	println!("Valid passwords without anagrams: {}", d4::CountAnagrams::count_valid(&contents));
}