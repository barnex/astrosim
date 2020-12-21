use super::bruteforce;
use super::prelude::*;
use std::mem::swap;

pub struct Stepper {
	particles: Vec<Particle>,
	pub target_error: f64,
	pub min_dt: f64,
	pub max_dt: f64,

	step_count: u64,
	time: f64,
	pub dt: f64,

	force: ForceFn,
	acc1: Vec<vec2>,
	acc2: Vec<vec2>,
}

// Force function, calculates accelartions due to the particle's interaction.
pub type ForceFn = Box<dyn Fn(&[Particle], &mut [vec2])>;

impl Stepper {
	pub fn new(mut particles: Vec<Particle>) -> Self {
		sort_by_mass(&mut particles);
		let cutoff = first_massless(&particles);
		Self::with_force(particles, move |p, a| bruteforce::set_accel_massless(p, a, cutoff))
	}

	pub fn with_force<F>(mut particles: Vec<Particle>, force: F) -> Self
	where
		F: Fn(&[Particle], &mut [vec2]) + 'static,
	{
		remove_net_momentum(&mut particles);

		// Set-up the initial accelartion once,
		// assumed initialized by step().
		let force = Box::new(force);
		let mut acc1 = zeros(particles.len());
		force(&particles, &mut acc1);

		Stepper {
			acc2: acc1.clone(),
			acc1,
			particles,
			step_count: 0,
			time: 0.0,
			dt: 1e-5,            // small initial time step, grows as needed, TODO
			target_error: 0.001, //TODO
			min_dt: 0.0,
			max_dt: INF,
			force,
		}
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

	pub fn fix_dt(&mut self, dt: f64) {
		self.dt = dt;
		self.min_dt = dt;
		self.max_dt = dt;
	}

	pub fn step_count(&self) -> u64 {
		self.step_count
	}

	/// Advance time by exactly total_time, without writing any output.
	pub fn advance(&mut self, total_time: f64) {
		// advance with no-op, no-error output function.
		self.advance_with_callback(total_time, |_| Ok(())).unwrap()
	}

	/// Advance time by exactly
	pub fn advance_with_output(&mut self, total_time: f64, outputs: &mut Outputs) -> Result<()> {
		self.advance_with_callback(total_time, |s| outputs.output(s))
	}

	/// Advance time by exactly total_time.
	/// Calls outfn(self) on each step, which may save output.
	fn advance_with_callback<F: FnMut(&Self) -> Result<()>>(&mut self, total_time: f64, mut outfn: F) -> Result<()> {
		// Output initial state
		if self.step_count == 0 {
			outfn(&self)?;
		}

		// Take normal time steps until just before the end time,
		// then take one last step, truncated to fit total_time exactly.
		let end_time = self.time + total_time;
		while self.time + self.dt < end_time {
			self.step_with_dt(self.dt);
			outfn(&self)?;
			self.adjust_dt();
		}
		let final_dt = end_time - self.time;
		if final_dt > 0.0 {
			self.step_with_dt(final_dt);
			outfn(&self)?;
			// truncated time step is not representative,
			// don't adjust dt based on it.
		}
		Ok(())
	}

	/// Take a single time step, with dt automatically adjusted
	/// based on the previous step's error estimate.
	pub fn step(&mut self) {
		if self.step_count != 0 {
			self.adjust_dt();
		}
		self.step_with_dt(self.dt);
	}

	// Take a single time step of size `dt`.
	// Acceleration must be up-to-date before step,
	// will be up-to-date after step (ready for next use).
	pub fn step_with_dt(&mut self, dt: f64) {
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
		self.step_count += 1;
	}

	//
	fn adjust_dt(&mut self) {
		let mut adjust = self.target_error / self.relative_error();
		adjust = f64::min(adjust, 1.4);
		adjust = f64::max(adjust, 0.1);
		self.dt *= adjust;
		self.dt = f64::max(self.dt, self.min_dt);
		self.dt = f64::min(self.dt, self.max_dt);
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
			let mut sim = Stepper::new(particles);
			sim.fix_dt(dt);
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
