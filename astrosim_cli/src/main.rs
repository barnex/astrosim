extern crate astrosim_lib;
extern crate serde;
extern crate structopt;
use astrosim_lib::prelude::*;
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
	#[structopt(long, default_value = "9e99")]
	max_dt: f64,

	/// Minimum integration time step.
	#[structopt(long, default_value = "1e-7")]
	min_dt: f64,

	/// Target relative error per oribit.
	/// TODO: steps per orbit? default 100?
	#[structopt(long, default_value = "0.03")]
	target_error: f64,

	/// Number of times to save the output.
	#[structopt(long, default_value = "300")]
	outputs: u32,

	/// Render this portion of the world.
	#[structopt(long, default_value = "2.0")]
	render_scale: f64,

	/// Render this number of pixels (for image width and height).
	#[structopt(long, default_value = "512")]
	render_pixels: u32,

	/// Enable writing timestep information to output_dir/timesteps.txt.
	#[structopt(long)]
	timesteps: bool,

	/// Write particle positions to output_dir/positions.txt every N time steps.
	#[structopt(long, short, default_value = "0")]
	positions_every: u32,

	/// Render particle positions as image at this interval.
	#[structopt(long, short, default_value = "0")]
	render_every: f64,

	/// Manually specify output directory.
	#[structopt(long, short)]
	output_dir: Option<String>,

	/// Do not remove net momentum from particles (allowing for systematic drift).
	#[structopt(long)]
	net_momentum: bool,
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

	let mut particles = load_particle_files(&args.files)?;

	//if !args.net_momentum {
	//	// A residual net momentum would cause a systematic drift.
	//	remove_net_momentum(&mut particles);
	//}

	//sort_by_mass(&mut particles);
	//println!("{:?}", &particles[0]);

	let output_dir = output_dir(&args);

	println!("input files:           {}", &args.files.join(","));
	println!("particles:             {}", particles.len());
	println!("net momentum removed:  {}", !args.net_momentum);
	println!("run time:              {}", args.time);
	println!("min time step:         {:e}", args.min_dt);
	println!("max time step:         {:e}", args.max_dt);
	println!("target relative error: {:e}", args.target_error);
	println!("output dir:            {}", &output_dir.to_string_lossy());
	println!("render every:          {} t", args.render_every);
	println!("positions every:       {} th time step", args.positions_every);
	println!("timesteps.txt:         {}", args.timesteps);

	let mut sim = Stepper::new(particles, BruteForce::boxed());
	sim.dt = args.initial_dt;
	sim.min_dt = args.min_dt;
	sim.max_dt = args.max_dt;
	sim.target_error = args.target_error;

	let mut outputs = Outputs::new(output_dir)? //
		.with_timesteps(args.timesteps)?
		.with_positions_every(args.positions_every)?
		.with_density(args.render_pixels)?
		.with_render_every(args.render_every)?;

	let start = std::time::Instant::now();

	sim.advance_with_output(args.time, &mut outputs)?;

	let duration = start.elapsed();
	let steps_per_sec = sim.step_count() as f64 / duration.as_secs_f64();

	outputs.close()?;

	println!("done in:               {:.2}s", duration.as_secs_f64());
	println!("steps per second:      {:.1}", steps_per_sec);

	Ok(())
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
