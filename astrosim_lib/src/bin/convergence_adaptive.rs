use astrosim_lib::prelude::*;

fn main() {
	let error = |target| {
		let particles = vec![Particle::new(1.0, vec2(0.0, 0.0), vec2(0.0, 0.0)), Particle::new(0.0, vec2(0.0, 1.0), vec2(1.0, 0.0))];

		let mut sim = Simulation::new(particles);
		sim.target_error = target;
		sim.advance(PI / 2.0);
		let got = sim.particles()[1].pos;
		let want = vec2(1.0, 0.0); // travelled a quarter orbit
		(sim.dt, (got - want).len())
	};

	let print_error = |target| {
		let (dt, error) = error(target);
		println!("{} {} {}", target, dt, error)
	};

	println!("#target_error dt final_error");
	for exp in 0..26 {
		let dt = 2.0f64.powf(-exp as f64);
		print_error(dt)
	}
}
