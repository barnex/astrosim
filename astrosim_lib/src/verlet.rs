use super::bruteforce;
use super::prelude::*;

pub fn advance<F>(accel: F, particles: &mut [Particle], total_time: f64, dt: f64)
where
	F: Fn(&[Particle], &mut [vec2]),
{
	// check that total_time and dt are not accidentally swapped
	if total_time < dt {
		panic!(
			"advance: total_time ({}) must be larger than dt ({})",
			total_time, dt
		)
	}

	let mut acc1 = zeros(particles.len()); // accelearation buffer
	let mut acc2 = zeros(particles.len()); // accelearation buffer
	let mut t = 0.0; // current time

	// take all but the very last time step

	let mut step = |dt| {
		accel(&particles, &mut acc1);
		for (i, p) in particles.iter_mut().enumerate() {
			p.vel += acc1[i] * (dt / 2.0);
			p.pos += p.vel * dt;
		}

		accel(&particles, &mut acc2);
		for (i, p) in particles.iter_mut().enumerate() {
			p.vel += acc2[i] * (dt / 2.0);
		}
	};

	while t + dt < total_time {
		step(dt);
		t += dt;
	}

	// the final time step may have to be truncated
	// to fit total_time precisely
	let final_dt = total_time - t;
	if final_dt > 0.0 {
		step(final_dt);
	}
}

// fn sqr(x: f64) -> f64 {
// 	x * x
// }

// pub fn step(particles: &mut [Particle], acc: &[vec2], dt: f64) {
// let dt_half = dt / 2.0;
// let dt_half2 = dt_half * dt_half;

// for (i, p) in particles.iter_mut().enumerate() {

// //p.vel = p.vel + acc[i] * dt;
// //p.pos = p.pos + p.vel * dt;

// }
// }

//  pub fn step_adaptive(particles: &mut [Particle], acc: &[vec2], dt: f64) -> f64{
//  	for (i, p) in particles.iter_mut().enumerate() {
//  		p.vel = p.vel + acc[i] * dt;
//  		p.pos = p.pos + p.vel * dt;
//  	}
//  }
//

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn verlet_error() {
		// position error after a quarter orbit
		let error = |dt| {
			let mut particles = vec![
				Particle::new(1.0, vec2(0.0, 0.0), vec2(0.0, 0.0)),
				Particle::new(0.0, vec2(0.0, 1.0), vec2(1.0, 0.0)),
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
}
