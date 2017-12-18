extern crate d17;

use d17::*;

fn solve_p2(nb_iterations:usize, max_value:i32) {
    let mut pos:usize = 0;
    let mut next_value = 0;

    for current_value in 1..max_value+1 {
        pos = (pos + nb_iterations) % (current_value as usize);
        // Every time we insert in position 0, the value after 0 changes
        if pos == 0 {
        	next_value = current_value;
        }
        pos = pos +1;
    }
    println!("{:?}", next_value);
}

fn main() {
    let nb_iterations = 370;
    let max_value = 2017;
    let v:Vec<i32> = generate_spinlock(nb_iterations, max_value);

    let pos_of_2017 =  v.iter().position(|&x| x == max_value).unwrap();
    println!("{:?}", v[pos_of_2017+1]);

    solve_p2(nb_iterations, 50000000);
}