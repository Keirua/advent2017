extern crate d7;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    //let mut f = File::open("d7.txt").expect("file not found");
    let mut f = File::open("d7.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    d7::parse_info(&contents);
}