use astrosim_lib::bruteforce;
use astrosim_lib::prelude::*;
use astrosim_lib::verlet;

fn main() {
	let error = |dt| {
		let mut particles = vec![
			Particle::new(1.0, vec2(0.0, 0.0), vec2(0.0, 0.0)),
			Particle::new(0.0, vec2(0.0, 1.0), vec2(1.0, 0.0)),
		];
		verlet::advance(bruteforce::set_accel, &mut particles, PI / 2.0, dt);
		let got = particles[1].pos;
		let want = vec2(1.0, 0.0); // travelled a quarter orbit
		(got - want).len()
	};

	let print_error = |dt| {
		let error = error(dt);
		println!("{} {}", dt, error)
	};

	for exp in 0..23 {
		let dt = 2.0f64.powf(-exp as f64);
		print_error(dt)
	}
}
