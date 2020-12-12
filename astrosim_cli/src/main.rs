extern crate astrosim_lib;
extern crate serde;
extern crate structopt;
use astrosim_lib::bruteforce;
use astrosim_lib::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
	/// Verlet integration time step.
	#[structopt(long, default_value = "0.001")]
	dt: f64,

	/// Files to process
	#[structopt(name = "FILE")]
	files: Vec<String>,
}

fn main() {
	let args = Args::from_args();

	if args.files.len() != 1 {
		fatal(&format!(
			"Need one input file (initial positions, velocities, mass), got {} files: {:?}",
			args.files.len(),
			&args.files
		));
	}
	let file = &args.files[0];
	let particles = match parse_file(file) {
		Ok(p) => p,
		Err(e) => fatal(&format!("parse {}: {}", file, e)),
	};

	for p in &particles {
		println!("{:?}", p)
	}
}

fn fatal(msg: &str) -> ! {
	eprintln!("{}", msg);
	std::process::exit(1);
}

type Err = Box<dyn Error>;
type Particles = Vec<Particle>;

fn parse_file(fname: &str) -> Result<Particles, Err> {
	parse(File::open(fname)?)
}

fn parse<R: Read>(input: R) -> Result<Particles, Err> {
	#[derive(Debug, Deserialize)]
	struct Record {
		pub m: f64,
		pub x: f64,
		pub y: f64,
		pub vx: f64,
		pub vy: f64,
	}
	let mut particles = Vec::new();
	let mut rdr = csv::Reader::from_reader(input);
	for result in rdr.deserialize() {
		let p: Record = result?;
		particles.push(Particle::new(p.m, vec2(p.x, p.x), vec2(p.vx, p.vy)));
	}
	Ok(particles)
}
