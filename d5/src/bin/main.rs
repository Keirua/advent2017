extern crate d5;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("d5.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    println!("nb_steps = {}", d5::find_nb_steps(&contents[..]));
    println!("nb_steps2 = {}", d5::find_nb_steps2(&contents[..]));
}