use super::outputs::*;
use super::prelude::*;
use std::cell::RefCell;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct SimWithOutput {
	sim: Simulation,
	out: Outputs,
}

impl SimWithOutput {
	pub fn new(sim: Simulation, output_dir: PathBuf, timesteps: bool, positions_every: u32) -> Result<Self> {
		let out = Outputs::new(output_dir, timesteps, positions_every)?;
		Ok(Self { sim, out })
	}

	pub fn advance(&mut self, total_time: f64) -> Result<()> {
		let (sim, out) = (&mut self.sim, &self.out);
		sim.advance_with_output(total_time, |s| out.do_output(s))
	}
}
