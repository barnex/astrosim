extern crate astrosim_lib;
extern crate serde;
extern crate structopt;
use astrosim_lib::bruteforce;
use astrosim_lib::prelude::*;
use astrosim_lib::verlet;
use serde::Deserialize;
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
	/// Total run time.
	#[structopt(long, default_value = "1.0")]
	t: f64,

	/// Verlet integration time step.
	#[structopt(long, default_value = "0.001")]
	dt: f64,

	/// Number of times to save the output.
	#[structopt(long, default_value = "100")]
	outputs: i32,

	/// Render this portion of the world.
	#[structopt(long, default_value = "2.0")]
	render_scale: f64,

	/// Render this number of pixels (for image width and height).
	#[structopt(long, default_value = "255")]
	render_pix: u32,

	/// Files to process
	#[structopt(name = "FILE")]
	files: Vec<String>,
}

fn main() {
	let args = Args::from_args();

	let mut particles = particles_from_args(&args);

	let total_time = args.t;
	let dt = args.dt;
	let num_outputs = args.outputs;
	for _i in 0..num_outputs {
		verlet::advance(
			bruteforce::set_accel,
			&mut particles,
			total_time / (num_outputs as f64),
			dt,
		);
		//bruteforce::set_accel(&particles, &mut acc);
		//verlet::step(&mut particles, &acc, args.dt);
		print_positions(&particles);
	}
}

fn print_positions(particles: &[Particle]) {
	for p in particles {
		print!("{} {} ", p.pos.x, p.pos.y);
	}
	println!("");
}

// construct particles list from command line arguments.
// TODO: concatenate multiple files
fn particles_from_args(args: &Args) -> Particles {
	if args.files.len() != 1 {
		fatal(&format!(
			"Need one input file (initial positions, velocities, mass), got {} files: {:?}",
			args.files.len(),
			&args.files
		));
	}
	let file = &args.files[0];
	match parse_particles_file(file) {
		Ok(p) => p,
		Err(e) => fatal(&format!("parse {}: {}", file, e)),
	}
}

type Particles = Vec<Particle>;

fn parse_particles_file(fname: &str) -> Result<Particles, Box<dyn Error>> {
	#[derive(Debug, Deserialize)]
	struct Record {
		pub m: f64,
		pub x: f64,
		pub y: f64,
		pub vx: f64,
		pub vy: f64,
	}
	let mut particles = Vec::new();
	let mut rdr = csv::ReaderBuilder::new()
		.trim(csv::Trim::All)
		.comment(Some(b'#'))
		.has_headers(false)
		.from_path(fname)?;
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
