extern crate image;
use super::prelude::*;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct Outputs {
	output_dir: PathBuf,

	render_every: f64,
	render_next: f64,

	density: Option<Image<f32>>,
	positions_every: u32,
	timestep_file: Option<BufWriter<File>>,
	positions_file: Option<BufWriter<File>>,
}

impl Outputs {
	const TIMESTEPS_FILE: &'static str = "timesteps.txt";
	const POSITIONS_FILE: &'static str = "positions.txt";
	const DENSITY_FILE: &'static str = "density.png";

	/// Outputs that will write to files in output_dir.
	/// Individual outputs still need to be enabled. E.g.:
	///
	///   let outputs = Outputs::new("some/dir")?
	///        .with_timesteps(true)?
	///        .with_positions_every(1)?;
	///
	pub fn new<P: AsRef<Path>>(output_dir: P) -> Result<Self> {
		fs::create_dir_all(&output_dir)?;

		Ok(Self {
			render_every: 0.0, // TODO
			positions_every: 0,
			output_dir: output_dir.as_ref().into(),
			timestep_file: None,
			positions_file: None,
			density: None,
			render_next: 0.0,
		})
	}

	/// Enables writing timesteps.txt file to output directory.
	pub fn with_timesteps(mut self, enabled: bool) -> Result<Self> {
		self.timestep_file = if enabled {
			let mut f = self.create(Self::TIMESTEPS_FILE)?;
			writeln!(f, "# time dt error")?;
			Some(f)
		} else {
			None
		};
		Ok(self)
	}

	/// Enables writing positions.txt files to output directory every `every`th time step.
	pub fn with_positions_every(mut self, every: u32) -> Result<Self> {
		self.positions_file = if every != 0 {
			let mut f = self.create(Self::POSITIONS_FILE)?;
			writeln!(f, "# time position_x position_y ...")?;
			Some(f)
		} else {
			None
		};
		Ok(self)
	}

	/// Enables writing a time-averaged density image of size pixels x pixels.
	pub fn with_density(mut self, pixels: u32) -> Result<Self> {
		self.density = if pixels != 0 { Some(Image::new(pixels, pixels)) } else { None };
		Ok(self)
	}

	///
	pub fn with_render_every(mut self, every: f64) -> Result<Self> {
		self.render_every = every;
		Ok(self)
	}

	pub fn close(self) -> Result<()> {
		//Self::render_density_file(&self.denSelf::DENSITY_FILE)?;
		// TODO: set everything to None, implement close on drop?
		Ok(())
	}

	/// To be called after every simulation time step
	/// to write all configured outputs.
	pub fn output(&mut self, sim: &Stepper) -> Result<()> {
		self.output_timesteps(sim)?;
		self.output_positions(sim)?;
		self.accumulate_density(sim);
		self.render(sim)?;
		Ok(())
	}

	fn accumulate_density(&mut self, sim: &Stepper) {
		if let Some(img) = self.density.as_mut() {
			let scale = 2.0; // TODO
				 // TODO: dt is wrong, is for next step should be for current
			accumulate_density(img, &sim.particles()[1..], scale, sim.dt() as f32)
		}
	}

	fn render(&mut self, sim: &Stepper) -> Result<()> {
		if let Some(img) = self.density.as_mut() {
			if self.render_every != 0.0 && sim.time() % self.render_every <= sim.dt {
				if sim.time() >= self.render_next {
					let file = self.output_dir.join(&format!("density_{:09}.png", sim.step_count()));
					Self::render_density_file(&img, &file)?;
					for p in img.pixels_mut() {
						*p = 0.0
					}
					self.render_next += self.render_every;
				}
			}
		}
		Ok(())
	}

	//fn render_positions(&mut self, particles: &[Particle], pixels: u32, scale: f64, i: u32) -> Result<()> {
	//	let path = format!("output/{:06}.png", i);

	//	let img_data = render::render(particles, pixels, scale);
	//	let img = image::ImageBuffer::from_fn(pixels, pixels, |x, y| {
	//		let v = img_data[y as usize][x as usize];
	//		let v = if v == 0.0 { 0u8 } else { 255u8 };
	//		image::Rgba([v, v, v, 255])
	//	});

	//	Ok(img.save(&path)?)
	//}

	fn render_density_file(density: &Image<f32>, file: &Path) -> Result<()> {
		let max = density.pixels().iter().fold(0.0, |a, b| f32::max(a, *b));
		let (w, h) = density.dimensions();
		let img = image::ImageBuffer::from_fn(w as u32, h as u32, |x, y| {
			let density = density[y as usize][x as usize];
			let v = ((density / max).sqrt() * 255.0) as u8;
			image::Rgba([v, v, v, 255])
		});
		img.save(file)?;
		Ok(())
	}

	fn output_positions(&mut self, sim: &Stepper) -> Result<()> {
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

	fn output_timesteps(&mut self, sim: &Stepper) -> Result<()> {
		if let Some(w) = self.timestep_file.as_mut() {
			writeln!(w, "{}\t{}\t{}", sim.time(), sim.dt(), sim.relative_error())?;
			w.flush()?;
		}
		Ok(())
	}

	// Create a file in the output directory.
	fn create(&self, basename: &str) -> Result<BufWriter<File>> {
		let name = self.output_dir.join(basename);
		let f = File::create(&name).msg(&format!("create {}", name.to_string_lossy()))?;
		let buf = BufWriter::new(f);
		Ok(buf)
	}
}
