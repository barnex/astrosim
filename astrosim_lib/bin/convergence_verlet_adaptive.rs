use astrosim_lib::adaptive;
use astrosim_lib::bruteforce;
use astrosim_lib::prelude::*;

fn main() {
	let error = |intial_dt, target_err| {
		let mut particles = vec![Particle::new(1.0, vec2(0.0, 0.0), vec2(0.0, 0.0)), Particle::new(0.0, vec2(0.0, 1.0), vec2(1.0, 0.0))];
		let (err, dt) = adaptive::advance(bruteforce::set_accel, &mut particles, PI / 2.0, intial_dt, target_err);
		//dbg!(intial_dt);
		//dbg!(dt);
		println!();
		let got = particles[1].pos;
		let want = vec2(1.0, 0.0); // travelled a quarter orbit
		(err, dt, (got - want).len())
	};

	let print_error = |initial_dt, target_err| {
		let (last_acc_err, dt, error) = error(initial_dt, target_err);
		println!("{} {} {} {}", target_err, dt, error, last_acc_err)
	};

	println!("# target_err  dt  global_error last_acc_error");
	for exp in 0..14 {
		let target_error = 2.0f64.powf(-exp as f64);
		let initial_dt = target_error / 1024.0;
		print_error(initial_dt, target_error)
	}
}
