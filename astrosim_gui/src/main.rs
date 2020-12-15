extern crate astrosim_lib;
use astrosim_lib::bruteforce;
use astrosim_lib::prelude::*;

fn main() {
	let p = vec![Particle::new(2.0, vec2(-1.0, 0.0), vec2(0.0, 0.0)), Particle::new(1.0, vec2(1.0, 0.0), vec2(0.0, 0.0))];

	let acc = bruteforce::accel(&p);
	acc.iter().for_each(|x| println!("{}", x));
}
