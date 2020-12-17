use astrosim_lib::prelude::*;

fn main() {
	let error = |dt| {
		let particles = vec![Particle::new(1.0, vec2(0.0, 0.0), vec2(0.0, 0.0)), Particle::new(0.0, vec2(0.0, 1.0), vec2(1.0, 0.0))];

		let mut sim = Simulation::new(particles);
		sim.fix_dt(dt);
		sim.advance(20. * PI);
		let got = sim.particles()[1].pos;
		let want = vec2(0.0, 1.0); // travelled one orbit
		sim.step(dt);
		let err_per_step = sim.relative_error();
		(err_per_step, (got - want).len())
	};

	let print_error = |dt| {
		let (err_per_step, error) = error(dt);
		println!("{} {} {}", dt, err_per_step, error)
	};

	println!("dt  error_per_step_estimate  total_error");
	for exp in 0..23 {
		let dt = 2.0f64.powf(-exp as f64);
		print_error(dt)
	}
}
