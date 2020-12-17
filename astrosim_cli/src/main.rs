extern crate astrosim_lib;
extern crate image;
extern crate serde;
extern crate structopt;
use astrosim_lib::prelude::*;
use astrosim_lib::render;
use serde::Deserialize;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
	/// Total run time.
	#[structopt(short, long, default_value = "6.28318530717959")]
	pub time: f64,

	/// Initial integration time step.
	#[structopt(short, long, default_value = "1e-5")]
	initial_dt: f64,

	/// Maximum integration time step.
	#[structopt(short, long, default_value = "9e99")]
	max_dt: f64,

	/// Minimum integration time step.
	#[structopt(short, long, default_value = "1e-7")]
	min_dt: f64,

	/// Target relative error per oribit.
	/// TODO: steps per orbit? default 100?
	#[structopt(long, default_value = "1e-3")]
	target_error: f64,

	/// Number of times to save the output.
	#[structopt(long, default_value = "300")]
	outputs: u32,

	/// Render this portion of the world.
	#[structopt(long, default_value = "2.0")]
	render_scale: f64,

	/// Render this number of pixels (for image width and height).
	#[structopt(long, default_value = "255")]
	render_pixels: u32,

	/// Enable writing timestep information to output_dir/timesteps.txt.
	#[structopt(long)]
	timesteps: bool,

	/// Write particle positions to output_dir/positions.txt every N time steps.
	#[structopt(long, short)]
	positions_every: u32,

	/// Manually specify output directory.
	#[structopt(long, short)]
	output_dir: Option<String>,

	/// Files to process
	#[structopt(name = "FILE")]
	files: Vec<String>,
}

fn main() {
	if let Err(e) = main_checked() {
		eprintln!("Error: {}", e);
		std::process::exit(1);
	}
}

fn main_checked() -> Result<()> {
	let args = Args::from_args();

	let particles = load_particle_files(&args.files)?;
	let output_dir = output_dir(&args);

	println!("input files:     {}", &args.files.join(","));
	println!("particles:       {}", particles.len());
	println!("run time:        {}", args.time);
	println!("output dir:      {}", &output_dir.to_string_lossy());
	println!("positions every: {}th time step", args.positions_every);
	println!("output timesteps:{}", args.timesteps);

	let mut sim = Simulation::new(particles);
	sim.dt = args.initial_dt;
	sim.min_dt = args.min_dt;
	sim.max_dt = args.max_dt;
	sim.target_error = args.target_error;

	let mut sim = sim.with_output(output_dir, args.timesteps, args.positions_every)?;
	sim.advance(args.time)?;

	Ok(())
}

fn render_positions(particles: &[Particle], pixels: u32, scale: f64, i: u32) -> Result<()> {
	let path = format!("output/{:06}.png", i);

	let img_data = render::render(particles, pixels, scale);
	let img = image::ImageBuffer::from_fn(pixels, pixels, |x, y| {
		let v = img_data[y as usize][x as usize];
		let v = if v == 0.0 { 0u8 } else { 255u8 };
		image::Rgba([v, v, v, 255])
	});

	Ok(img.save(&path)?)
}

// output directory: first input file, but with extension ".out",
// unless explicitly overridden by flag --output-dir.
fn output_dir(args: &Args) -> PathBuf {
	if let Some(dir) = &args.output_dir {
		PathBuf::from(dir)
	} else {
		PathBuf::from(&args.files[0]).with_extension("out")
	}
}

// Load particles from one or more CSV files.
// Particles from multiple files are concatenated.
// Zero files is an error.
fn load_particle_files(files: &[String]) -> Result<Vec<Particle>> {
	if files.len() == 0 {
		return err("need at least one input file (CSV with mass, positions, velocities)");
	}
	let mut particles = Vec::new();
	for file in files {
		particles.append(&mut load_particle_file(file)?)
	}
	Ok(particles)
}

// Load particles from a CSV file with columns:
//
// 	mass, position_x, position_y, velocity_x, velocity_y
//
// Comment character is `#`.
fn load_particle_file(fname: &str) -> Result<Vec<Particle>> {
	#[derive(Debug, Deserialize)]
	struct Record {
		pub m: f64,
		pub x: f64,
		pub y: f64,
		pub vx: f64,
		pub vy: f64,
	}
	let mut particles = Vec::new();
	let msg = format!("load particles: {}", fname);
	let mut rdr = csv::ReaderBuilder::new() //
		.trim(csv::Trim::All)
		.comment(Some(b'#'))
		.has_headers(false)
		.from_path(fname)
		.msg(&msg)?;
	for result in rdr.deserialize() {
		let p: Record = result.msg(&msg)?;
		particles.push(Particle::new(p.m, vec2(p.x, p.y), vec2(p.vx, p.vy)));
	}
	Ok(particles)
}
