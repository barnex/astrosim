extern crate astrosim_lib;
extern crate image;
extern crate serde;
extern crate structopt;
use astrosim_lib::bruteforce;
use astrosim_lib::prelude::*;
use astrosim_lib::render;
use astrosim_lib::verlet;
use serde::Deserialize;
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
	/// Total run time.
	#[structopt(short, long, default_value = "1.0")]
	pub time: f64,

	/// Verlet integration time step.
	#[structopt(short, long, default_value = "0.001")]
	dt: f64,

	/// Target relative error per oribit.
	/// TODO: steps per orbit? default 100?
	#[structopt(short, long, default_value = "0.001")]
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

	/// Files to process
	#[structopt(name = "FILE")]
	files: Vec<String>,
}

fn main() {
	let args = Args::from_args();

	let mut particles = check("parse particles input files", particles_from_args(&args));

	let total_time = args.time;
	let dt = args.dt;
	let num_outputs = args.outputs;
	for i in 0..num_outputs {
		verlet::advance(bruteforce::set_accel, &mut particles, total_time / (num_outputs as f64), dt);
		//print_positions(&particles);
		check("render", render_positions(&particles, args.render_pixels, args.render_scale, i))
	}
}

fn check<V>(msg: &str, result: Result<V, Err>) -> V {
	match result {
		Err(e) => fatal(&format!("{}: {}", msg, e)),
		Ok(v) => v,
	}
}

fn render_positions(particles: &[Particle], pixels: u32, scale: f64, i: u32) -> Result<(), Err> {
	let path = format!("output/{:06}.png", i);

	let img_data = render::render(particles, pixels, scale);
	let img = image::ImageBuffer::from_fn(pixels, pixels, |x, y| {
		let v = img_data[y as usize][x as usize];
		let v = if v == 0.0 { 0u8 } else { 255u8 };
		image::Rgba([v, v, v, 255])
	});

	Ok(img.save(&path)?)
}

fn print_positions(particles: &[Particle]) {
	for p in particles {
		print!("{} {} ", p.pos.x, p.pos.y);
	}
	println!("");
}

// construct particles list from command line arguments.
// TODO: concatenate multiple files
fn particles_from_args(args: &Args) -> Result<Particles, Err> {
	if args.files.len() == 0 {
		fatal("Need at least one input file (CSV with mass, positions, velocities)");
	}
	let mut particles = Vec::new();
	for file in &args.files {
		particles.append(&mut parse_particles_file(file)?)
	}
	Ok(particles)
}

type Particles = Vec<Particle>;
type Err = Box<dyn Error>;

fn parse_particles_file(fname: &str) -> Result<Particles, Err> {
	#[derive(Debug, Deserialize)]
	struct Record {
		pub m: f64,
		pub x: f64,
		pub y: f64,
		pub vx: f64,
		pub vy: f64,
	}
	let mut particles = Vec::new();
	let mut rdr = csv::ReaderBuilder::new().trim(csv::Trim::All).comment(Some(b'#')).has_headers(false).from_path(fname)?;
	for result in rdr.deserialize() {
		let p: Record = result?;
		particles.push(Particle::new(p.m, vec2(p.x, p.y), vec2(p.vx, p.vy)));
	}
	Ok(particles)
}

fn fatal(msg: &str) -> ! {
	eprintln!("{}", msg);
	std::process::exit(1);
}
