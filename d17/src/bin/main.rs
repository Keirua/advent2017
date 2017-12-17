extern crate d17;

use d17::*;

fn main() {
    let nb_iterations = 3;
    let max_value = 4;
    let v:Vec<i32> = generate_spinlock(nb_iterations, max_value);
/*
    let mut pos:usize = 0;
    v.push(0);
    for current_value in 1..max_value+1 {
    	pos = (pos + nb_iterations) % v.len();
    	v.insert(pos, current_value);
    	pos += 1;
    	println!("{:?} {:?}", pos, v);
    }*/

    println!("{:?}", v);
}