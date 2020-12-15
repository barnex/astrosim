use super::prelude::*;

pub fn advance<F>(accel: F, particles: &mut [Particle], total_time: f64, dt: f64)
where
	F: Fn(&[Particle], &mut [vec2]), // acceleration function
{
	check_dt(total_time, dt);
	let mut acc = zeros(particles.len()); // accelearation buffer
	accel(&particles, &mut acc); // initial acceleration to seed step 1.

	let mut t = 0.0; // current time

	// take all but the final time step,
	// so that we definitely do not step too far.
	while t + dt < total_time {
		step(&accel, particles, &mut acc, dt);
		t += dt;
	}
	// the final time step may have to be truncated
	// to fit total_time precisely
	let final_dt = total_time - t;
	if final_dt > 0.0 {
		step(&accel, particles, &mut acc, final_dt);
	}
}

// Take a single leapfrog integration step
// (https://en.wikipedia.org/wiki/Leapfrog_integration).
//
// Uses "kick-drift-kick" form, allowing for a variable time step.
//
// This method has the "First Same As Last" (FSAL) property:
// The acceleration at the beginning of step N is identical
// to the last accelaration of step N-1, and need not be re-calcualted
// (except before the very first step).
//
// `acc` must contain the accelerations at the beginning of the step,
// will be overwritten by the accelerations at the end of the step
// (thus ready for re-use by the next step).
pub fn step<F>(accel: F, particles: &mut [Particle], acc: &mut [vec2], dt: f64)
where
	F: Fn(&[Particle], &mut [vec2]), // acceleration function
{
	// stage 1: 1/2 kick + drift
	// re-use the last acceleration from the previous step.
	for (i, p) in particles.iter_mut().enumerate() {
		p.vel += acc[i] * (dt / 2.0);
		p.pos += p.vel * dt;
	}

	// stage 2: update acceleration, do 1/2 kick
	accel(&particles, acc);
	for (i, p) in particles.iter_mut().enumerate() {
		p.vel += acc[i] * (dt / 2.0);
	}
}

// check that total_time and dt are not accidentally swapped
pub fn check_dt(total_time: f64, dt: f64) {
	if total_time < dt {
		panic!("advance: total_time ({}) must be larger than dt ({})", total_time, dt)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::bruteforce;

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
}
