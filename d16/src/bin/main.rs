extern crate d16;
use d16::*;

fn main() {
    let p = "s1,x3/4,pe/b";
    let program = extract_program(p);
    println!("{:?}", program);
}