
extern crate d10;
use d10::*;
use std::str::FromStr;

fn perform_hash(v: &mut Vec<i32>, operations: Vec<i32>) {
	let mut current_position:i32 = 0;
	let mut current_skip:i32 = 0;
	for op in operations {
		rev(v, current_position as usize, op as usize);
		println!("{} {:?}", op, v);
		current_position += op + current_skip;
		current_skip += 1;
	}
}

fn main() {
	/*
    let mut v = vec![0,1,2,3,4,5,6];
    println!("{:?}", v);
    rev(&mut v, 5, 3);
    // [5, 1, 2, 3, 4, 0, 6]
    println!("{:?}", v);
    */
    /*
    let operations = vec![3,4,1,5];
    let mut v = vec![0,1,2,3,4];
    perform_hash(&mut v, operations);
    */
    // let mut v = vec![0,1,2,3,4,5,6];
    let mut v:Vec<i32> = Vec::with_capacity(256);
    for i in 0..256 {
	    v.push(i);
	}
	
    println!("{:?}", v);
    let input = "34,88,2,222,254,93,150,0,199,255,39,32,137,136,1,167";
    let operations = input.split(",")
		.map(|x| i32::from_str(x).unwrap())
		.collect::<Vec<i32>>();
	perform_hash(&mut v, operations);
	println!("{}", v[0] * v[1]);
}