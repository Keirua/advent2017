extern crate d17;

use d17::*;

fn main() {
    let nb_iterations = 370;
    let max_value = 2017;
    let v:Vec<i32> = generate_spinlock(nb_iterations, max_value);

    println!("{:?}", v);
}