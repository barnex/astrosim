extern crate rand;
use astrosim_lib::prelude::*;
use std::fs;
use std::path::PathBuf;

const NUM_ASTEROIDS: usize = 10000;
fn main() -> Result<()> {
	let dir = PathBuf::from("kirwood_gaps.out");
	fs::create_dir_all(&dir)?;

	let particles = init_particles();
	let mut sim = Simulation::new(particles);

	//let mut out = Outputs::new("kirwood_gaps.out")?;

	let (w, h) = (512, 512);
	let mut img = Image::<f32>::new(w, h);

	let scale = 2.0;
	for i in 0..10000 {
		sim.step();
		accumulate_density(&mut img, &sim.particles()[1..], scale, sim.dt() as f32);
	}
	save_density(&img, &dir.join("density_final.png"))?;

	Ok(())
}

fn init_particles() -> Vec<Particle> {
	let mut particles = vec![
		Particle::new(1.0, vec2(0.0, 0.0), vec2(0.0, 0.0)),  // sun
		Particle::new(1e-3, vec2(1.0, 0.0), vec2(0.0, 0.1)), // jupiter
	];
	particles.append(&mut asteroids(NUM_ASTEROIDS, 0.5, 1.5));
	particles
}

fn asteroids(n: usize, rmin: f64, rmax: f64) -> Vec<Particle> {
	let mut particles = Vec::with_capacity(n);

	for i in 0..n {
		let dr = i as f64 / n as f64;
		let r = f64::sqrt(rmin + dr * (rmax - rmin));

		let theta = rnd(0.0, 2.0 * PI);
		let v = f64::sqrt(1.0 / r);
		let x = f64::cos(theta);
		let y = f64::sin(theta);
		let pos = r * vec2(x, y);
		let vel = v * vec2(-y, x);
		let mass = 0.0;
		particles.push(Particle::new(mass, pos, vel));
	}
	particles
}

fn rnd(min: f64, max: f64) -> f64 {
	min + rand::random::<f64>() * (max - min)
}
