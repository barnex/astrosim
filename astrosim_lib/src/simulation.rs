use super::bruteforce;
use super::prelude::*;
use std::cell::RefCell;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::mem::swap;
use std::path::{Path, PathBuf};

pub struct Simulation {
	particles: Vec<Particle>,
	step: u64,
	time: f64,
	dt: f64,
	target_error: f64,
	force: ForceFn,
	acc1: Vec<vec2>,
	acc2: Vec<vec2>,

	output_dir: PathBuf,
	render_every: f64,
	positions_every: u32,
	timestep_file: Option<RefCell<BufWriter<File>>>,
	positions_file: Option<RefCell<BufWriter<File>>>,
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
			render_every: 0.0,
			positions_every: 0,
			output_dir: PathBuf::new(),
			timestep_file: None,
			positions_file: None,
		}
	}

	/// Enable writing periodic output.
	pub fn with_output(mut self, output_dir: PathBuf, timesteps: bool, positions_every: u32) -> Result<Self> {
		fs::create_dir_all(&output_dir)?;
		self.output_dir = output_dir;

		if timesteps {
			let mut f = self.create(Self::TIMESTEPS_FILE)?;
			writeln!(f, "# time dt error")?;
			self.timestep_file = Some(RefCell::new(f));
		}

		self.positions_every = positions_every;
		if positions_every != 0 {
			let mut f = self.create(Self::POSITIONS_FILE)?;
			writeln!(f, "# time position_x position_y ...")?;
			self.positions_file = Some(RefCell::new(f));
		}

		Ok(self)
	}

	const TIMESTEPS_FILE: &'static str = "timesteps.txt";
	const POSITIONS_FILE: &'static str = "positions.txt";

	// Create a file in the output directory.
	fn create(&self, basename: &str) -> Result<BufWriter<File>> {
		let name = self.output_dir.join(basename);
		let f = File::create(&name).msg(&format!("create {}", name.to_string_lossy()))?;
		let buf = BufWriter::new(f);
		Ok(buf)
	}

	pub fn with_render_every(mut self, dt: f64) -> Self {
		self.render_every = dt;
		self
	}

	pub fn particles(&self) -> &[Particle] {
		&self.particles
	}

	/// Advance time by exactly total_time, without writing any output.
	/// Intended for tests.
	pub fn advance(&mut self, total_time: f64) {
		// advance with no-op, no-error output function.
		self.advance_(total_time, |_| Ok(())).unwrap()
	}

	/// Advance time by exactly total_time.
	/// Write output files if configured so.
	pub fn advance_with_output(&mut self, total_time: f64) -> Result<()> {
		self.advance_(total_time, |s| s.do_output())
	}

	/// Advance time by exactly total_time.
	/// Calls outfn(self) on each step, which may save output.
	fn advance_<F: Fn(&Self) -> Result<()>>(&mut self, total_time: f64, outfn: F) -> Result<()> {
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
		// let mut adjust = self.target_error / self.relative_error();
		// if adjust > 1.4 {
		// 	adjust = 1.4;
		// }
		// if adjust < 0.1 {
		// 	adjust = 0.1;
		// }
		// self.dt *= adjust;
	}

	fn relative_error(&self) -> f64 {
		self.acc1
			.iter()
			.zip(self.acc2.iter())
			.map(|(a1, a2)| (*a1 - *a2).len2() / (*a1 + *a2).len2())
			.fold(0.0, |max, val| f64::max(max, val))
			.sqrt() * 2.0
	}

	fn do_output(&self) -> Result<()> {
		self.output_timesteps()?;
		self.output_positions()?;
		Ok(())
	}

	fn output_positions(&self) -> Result<()> {
		if self.positions_every != 0 && self.step % (self.positions_every as u64) == 0 {
			if let Some(cell) = self.positions_file.as_ref() {
				let mut w = cell.borrow_mut();
				write!(w, "{}", self.time)?;
				for p in self.particles() {
					write!(w, " {} {}", p.pos.x, p.pos.y)?;
				}
				writeln!(w)?;
				w.flush()?;
			}
		}
		Ok(())
	}

	fn output_timesteps(&self) -> Result<()> {
		if let Some(cell) = self.timestep_file.as_ref() {
			let mut w = cell.borrow_mut();
			writeln!(w, "{}\t{}\t{}", self.time, self.dt, self.relative_error())?;
			w.flush()?;
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
