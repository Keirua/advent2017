extern crate d8;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("d8.txt").expect("file not found");

    let mut code = String::new();
    f.read_to_string(&mut code)
        .expect("something went wrong reading the file");
    /*let code = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";*/

	let (operations, mut registers) = d8::extract_operation_list(&code[..]);
	d8::apply_operations(&operations, &mut registers);
	// println!("{:#?}", registers);

	let mut min_value:Option<i32> = None;
	for (_, &value) in registers.iter() {
		match min_value {
			None => { min_value = Some(value); },
			Some(best_min_value) => {
				if value > best_min_value {
					min_value = Some(value);
				}
			}
		}
    }
    println!("{:?}", min_value);
}