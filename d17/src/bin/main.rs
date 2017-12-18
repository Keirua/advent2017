extern crate d17;

use d17::*;

fn main() {
    let nb_iterations = 370;
    let max_value = 2017;
    let v:Vec<i32> = generate_spinlock(nb_iterations, max_value);


    let pos_of_2017 =  v.iter().position(|&x| x == 2017).unwrap();
    println!("{:?}", v[pos_of_2017+1]);

    let v2:Vec<i32> = generate_spinlock(nb_iterations, 50000000);
    let pos_of_0 =  v2.iter().position(|&x| x == 0).unwrap();
    println!("{:?}", v2[pos_of_0+1]);
}