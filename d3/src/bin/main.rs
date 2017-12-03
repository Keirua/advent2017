extern crate d3;

pub fn main() {
	let v = 347991;
	//let v = 23;
    let coords = d3::generate_coords(v);
    // println!("{:?}", coords[v].manhattan(&coords[1]));

    let g = d3::Grid::generate_sum_grid(v);
    // println!("{}", g);
}