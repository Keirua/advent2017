extern crate d10;
use d10::*;

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
    let s = "34,88,2,222,254,93,150,0,199,255,39,32,137,136,1,167";
    let operations = generate_operation_list(&s);
    perform_hash1(&mut v, operations);
    // Solution to part 1
    println!("{}", v[0] * v[1]);

    // Solution to part 2
    println!("{:?}", generate_hash(s));
}