extern crate astrosim_lib;
use astrosim_lib::prelude::*;
//use astrosim_cli::*;

fn main() {
	let mut u = Universe::new();
	u.push(Particle::new(1.0, vec2(2.0, 3.0), vec2(4.0, 5.0)))
}
