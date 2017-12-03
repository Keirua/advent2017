extern crate d3;

pub fn main() {
	let v = 347991;
    let coords = d3::generate_coords(v);
    println!("{:?}", coords[v].manhattan(&coords[1]));
}