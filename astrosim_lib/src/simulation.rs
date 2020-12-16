use super::bruteforce;
use super::prelude::*;
use std::cell::RefCell;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::mem::swap;
use std::path::{Path, PathBuf};

pub struct Simulation {
	particles: Vec<Particle>,
	step: u64,
	time: f64,
	dt: f64,
	force: ForceFn,
	acc1: Vec<vec2>,
	acc2: Vec<vec2>,

	output_dir: PathBuf,
	render_every: f64,
	output_table: Option<RefCell<File>>,
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
			force,
			render_every: 0.0,
			output_dir: PathBuf::new(),
			output_table: None,
		}
	}

	/// Enable writing periodic output.
	pub fn with_output(mut self, output_dir: PathBuf, timesteps: bool) -> Result<Self> {
		fs::create_dir_all(&output_dir)?;
		self.output_dir = output_dir;

		if timesteps {
			let f = self.output_dir.join(Self::TABLE_FILE);
			let mut f = File::create(&f).msg(&format!("output table: create {}", f.to_string_lossy()))?;
			writeln!(f, "# time dt error")?;
			self.output_table = Some(RefCell::new(f));
		}

		Ok(self)
	}

	const TABLE_FILE: &'static str = "timesteps.txt";

	pub fn with_render_every(mut self, dt: f64) -> Self {
		self.render_every = dt;
		self
	}

	pub fn particles(&self) -> &[Particle] {
		&self.particles
	}

	// Advance time by exactly total_time.
	pub fn advance(&mut self, total_time: f64) -> Result<()> {
		// Output initial state
		self.do_output()?;

		// Take normal time steps until just before the end time,
		// then take one last step, truncated to fit total_time exactly.
		let end_time = self.time + total_time;
		while self.time + self.dt < end_time {
			self.step(self.dt);
			self.do_output()?;
			self.update_dt();
		}
		let final_dt = end_time - self.time;
		if final_dt > 0.0 {
			self.step(final_dt);
			self.do_output()?;
			// truncated time step is not representative,
			// don't update dt based on it.
		}
		Ok(())
	}

	// Take a single time step of size `dt`.
	// Acceleration must be up-to-date before step,
	// will be up-to-date after step (ready for next use).
	fn step(&mut self, dt: f64) {
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
		//let adjust =
	}

	fn relative_error(&self) -> f64 {
		self.acc1
			.iter()
			.zip(self.acc2.iter())
			.map(|(a1, a2)| (*a1 - *a2).len2() / (0.25 * (*a1 + *a2).len2()))
			.fold(0.0, |max, val| f64::max(max, val))
			.sqrt()
	}

	fn do_output(&self) -> Result<()> {
		if let Some(cell) = self.output_table.as_ref() {
			let mut w = cell.borrow_mut();
			writeln!(w, "{}\t{}\t{}", self.time, self.dt, self.relative_error())?
		}
		Ok(())
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
