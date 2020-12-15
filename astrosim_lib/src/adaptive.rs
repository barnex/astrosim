use super::prelude::*;
use super::verlet;

// returns new dt
pub fn advance<F>(accel: F, particles: &mut [Particle], total_time: f64, initial_dt: f64, max_error: f64) -> (f64, f64)
where
	F: Fn(&[Particle], &mut [vec2]), // acceleration function
{
	verlet::check_dt(total_time, initial_dt);

	let mut acc1 = zeros(particles.len());
	let mut acc2 = zeros(particles.len());
	accel(&particles, &mut acc1);

	let mut step = |initial_dt| {
		let dt = initial_dt / 2.0; // because we do 2 steps in one

		// 1/2 kick + drift
		// re-use the last acceleration from the previous step.
		for (i, p) in particles.iter_mut().enumerate() {
			p.vel += acc1[i] * (dt / 2.0);
			p.pos += p.vel * dt;
		}

		// update acceleration, do 1 kick + 1 drift
		accel(&particles, &mut acc2);
		for (i, p) in particles.iter_mut().enumerate() {
			p.vel += acc2[i] * dt;
			p.pos += p.vel * dt;
		}

		// 1/2 kick
		accel(&particles, &mut acc1);
		for (i, p) in particles.iter_mut().enumerate() {
			p.vel += acc1[i] * (dt / 2.0);
		}

		let curr_error = max_delta_acc(&acc1, &acc2);
		//dbg!(curr_error);
		let adjust = max_error / curr_error;
		let new_dt = initial_dt * adjust;
		(curr_error, new_dt)
	};

	let mut t = 0.0; // current time
	let mut dt = initial_dt;
	let mut err = 0.0;
	// take all but the final time step,
	// so that we definitely do not step too far.
	while t + dt < total_time {
		let (err_, dt_) = step(dt);
		err = err_;
		dt = dt_;
		t += dt;
	}
	// the final time step may have to be truncated
	// to fit total_time precisely
	let final_dt = total_time - t;
	if final_dt > 0.0 {
		step(final_dt);
	}
	(err, dt)
}

fn max_delta_acc(acc1: &[vec2], acc2: &[vec2]) -> f64 {
	acc1.iter()
		.zip(acc2.iter())
		.map(|(a1, a2)| (*a1 - *a2).len2() / (0.25 * (*a1 + *a2).len2()))
		.fold(0.0, |m, v| f64::max(m, v))
		.sqrt()
}

#[cfg(test)]
mod test {
	use super::*;

	/*
	#[test]
	fn delta_acc() {
		let acc1 = vec![vec2(1.0, 2.0), vec2(3.0, 4.0)];
		let acc2 = vec![vec2(1.1, 2.2), vec2(3.0, 3.5)];
		assert_eq!(max_delta_acc(&acc1, &acc2), 0.5);
	}
	*/

	// TODO: Test scale-independent errors, test elliptical orbit.

	/*
	#[test]
	fn verlet_error() {
		// position error after a quarter orbit
		let error = |dt| {
			let mut particles = vec![
				Particle::new(1.0, vec2(0.0, 0.0), vec2(0.0, 0.0)), // "sun"
				Particle::new(0.0, vec2(0.0, 1.0), vec2(1.0, 0.0)), // "earth"
			];
			advance(bruteforce::set_accel, &mut particles, PI / 2.0, dt);
			let got = particles[1].pos;
			let want = vec2(1.0, 0.0); // travelled a quarter orbit
			(got - want).len()
		};

		let check = |dt, tolerance| {
			let error = error(dt);
			if error > tolerance {
				panic!("dt {}: error {} > tolerance {}", dt, error, tolerance)
			}
		};

		// errors needs drop quadratically with time step
		// the prefactor is chosen as tightly as possible
		// to be sensitive to regressions.
		check(1e-1, 3e-3);
		check(1e-2, 3e-5);
		check(1e-3, 3e-7);
		check(1e-4, 3e-9);
	}
	*/
}
