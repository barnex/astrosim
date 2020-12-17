use super::prelude::*;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct Outputs {
	output_dir: PathBuf,
	render_every: f64,
	positions_every: u32,
	timestep_file: Option<BufWriter<File>>,
	positions_file: Option<BufWriter<File>>,
}

impl Outputs {
	const TIMESTEPS_FILE: &'static str = "timesteps.txt";
	const POSITIONS_FILE: &'static str = "positions.txt";

	pub fn new(output_dir: PathBuf, timesteps: bool, positions_every: u32) -> Result<Self> {
		fs::create_dir_all(&output_dir)?;

		let timestep_file = if timesteps {
			let mut f = Self::create(&output_dir, Self::TIMESTEPS_FILE)?;
			writeln!(f, "# time dt error")?;
			Some(f)
		} else {
			None
		};

		let positions_file = if positions_every != 0 {
			let mut f = Self::create(&output_dir, Self::POSITIONS_FILE)?;
			writeln!(f, "# time position_x position_y ...")?;
			Some(f)
		} else {
			None
		};

		Ok(Self {
			render_every: 0.0, // TODO
			positions_every,
			output_dir,
			timestep_file,
			positions_file,
		})
	}

	// Create a file in the output directory.
	fn create(output_dir: &Path, basename: &str) -> Result<BufWriter<File>> {
		let name = output_dir.join(basename);
		let f = File::create(&name).msg(&format!("create {}", name.to_string_lossy()))?;
		let buf = BufWriter::new(f);
		Ok(buf)
	}

	pub fn do_output(&mut self, sim: &Simulation) -> Result<()> {
		self.output_timesteps(sim)?;
		self.output_positions(sim)?;
		Ok(())
	}

	fn output_positions(&mut self, sim: &Simulation) -> Result<()> {
		if self.positions_every != 0 && sim.step_count() % (self.positions_every as u64) == 0 {
			if let Some(w) = self.positions_file.as_mut() {
				write!(w, "{}", sim.time())?;
				for p in sim.particles() {
					write!(w, " {} {}", p.pos.x, p.pos.y)?;
				}
				writeln!(w)?;
				w.flush()?;
			}
		}
		Ok(())
	}

	fn output_timesteps(&mut self, sim: &Simulation) -> Result<()> {
		if let Some(w) = self.timestep_file.as_mut() {
			writeln!(w, "{}\t{}\t{}", sim.time(), sim.dt(), sim.relative_error())?;
			w.flush()?;
		}
		Ok(())
	}
}
