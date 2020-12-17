use super::bruteforce;
use super::prelude::*;
use std::mem::swap;
use std::path::PathBuf;

pub struct Simulation {
	particles: Vec<Particle>,
	step: u64,
	time: f64,
	dt: f64,
	target_error: f64,
	force: ForceFn,
	acc1: Vec<vec2>,
	acc2: Vec<vec2>,
}

// Force function, calculates accelartions due to the particle's interaction.
pub type ForceFn = Box<dyn Fn(&[Particle], &mut [vec2])>;

impl Simulation {
	pub fn new(particles: Vec<Particle>, dt: f64) -> Self {
		// Set-up the initial accelartion once,
		// assumed initialized by step().
		let force = Box::new(bruteforce::set_accel);
		let mut acc1 = zeros(particles.len());
		force(&particles, &mut acc1);

		Self {
			acc2: acc1.clone(),
			acc1,
			particles,
			step: 0,
			time: 0.0,
			dt,
			target_error: 0.001, //TODO
			force,
		}
	}

	/// Enable writing periodic output.
	pub fn with_output(self, output_dir: PathBuf, timesteps: bool, positions_every: u32) -> Result<SimWithOutput> {
		SimWithOutput::new(self, output_dir, timesteps, positions_every)
	}

	pub fn particles(&self) -> &[Particle] {
		&self.particles
	}

	pub fn time(&self) -> f64 {
		self.time
	}

	pub fn dt(&self) -> f64 {
		self.dt
	}

	pub fn step(&self) -> u64 {
		self.step
	}

	/// Advance time by exactly total_time, without writing any output.
	/// Intended for tests.
	pub fn advance(&mut self, total_time: f64) {
		// advance with no-op, no-error output function.
		self.advance_with_output(total_time, |_| Ok(())).unwrap()
	}

	/// Advance time by exactly total_time.
	/// Calls outfn(self) on each step, which may save output.
	pub fn advance_with_output<F: Fn(&Self) -> Result<()>>(&mut self, total_time: f64, outfn: F) -> Result<()> {
		// Output initial state
		//self.do_output()?;
		outfn(&self)?;

		// Take normal time steps until just before the end time,
		// then take one last step, truncated to fit total_time exactly.
		let end_time = self.time + total_time;
		while self.time + self.dt < end_time {
			self.step_no_output(self.dt);
			//	self.do_output()?;
			outfn(&self)?;
			self.update_dt();
		}
		let final_dt = end_time - self.time;
		if final_dt > 0.0 {
			self.step_no_output(final_dt);
			//	self.do_output()?;
			outfn(&self)?;
			// truncated time step is not representative,
			// don't update dt based on it.
		}
		Ok(())
	}

	// Take a single time step of size `dt`.
	// Acceleration must be up-to-date before step,
	// will be up-to-date after step (ready for next use).
	//
	// Does not write output files.
	fn step_no_output(&mut self, dt: f64) {
		// https://en.wikipedia.org/wiki/Leapfrog_integration#Algorithm, "synchronized" form.

		// "drift" the positions with previous velocities and acceleration.
		for (i, p) in self.particles.iter_mut().enumerate() {
			let a1 = self.acc1[i];
			p.pos += p.vel * dt + 0.5 * a1 * dt * dt;
		}

		// update acc2
		(self.force)(&self.particles, &mut self.acc2);

		// "kick" the velocity with the average accelartion over the step.
		for (i, p) in self.particles.iter_mut().enumerate() {
			let a1 = self.acc1[i];
			let a2 = self.acc2[i];
			p.vel += 0.5 * (a1 + a2) * dt;
		}
		// swap so that acc1 holds the acceleration for the next time step.
		swap(&mut self.acc1, &mut self.acc2);
		self.time += dt;
		self.step += 1;
	}

	fn update_dt(&mut self) {
		// works but test must be updated
		let mut adjust = self.target_error / self.relative_error();
		if adjust > 1.4 {
			adjust = 1.4;
		}
		if adjust < 0.1 {
			adjust = 0.1;
		}
		self.dt *= adjust;
	}

	pub fn relative_error(&self) -> f64 {
		self.acc1
			.iter()
			.zip(self.acc2.iter())
			.map(|(a1, a2)| (*a1 - *a2).len2() / (*a1 + *a2).len2())
			.fold(0.0, |max, val| f64::max(max, val))
			.sqrt() * 2.0
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn verlet_error() {
		// position error after a quarter orbit
		let error = |dt| {
			let particles = vec![
				Particle::new(1.0, vec2(0.0, 0.0), vec2(0.0, 0.0)), // "sun"
				Particle::new(0.0, vec2(0.0, 1.0), vec2(1.0, 0.0)), // "earth"
			];
			let mut sim = Simulation::new(particles, dt);
			sim.advance(PI / 2.0);
			let got = sim.particles()[1].pos;
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
