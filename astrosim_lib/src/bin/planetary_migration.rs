extern crate rand;
use astrosim_lib::prelude::*;
use std::fs;
use std::path::PathBuf;

const NUM_ASTEROIDS: usize = 100;
const ASTEROIDS_MASS: f64 = 3e-7;

fn main() -> Result<()> {
	let dir = PathBuf::from("planetary_migration.out");
	fs::create_dir_all(&dir)?;

	let particles = init_particles();
	let mut sim = Stepper::new(particles, PartialForce::new(2)); // TODO: fix sun
	sim.target_error = 0.001;
	sim.min_dt = 0.0001;

	let (w, h) = (512, 512); // TODO: non-square crashes
	let mut img = Image::<f32>::new(w, h);
	let scale = 2.5;

	for i in 0..100000 {
		println!("{}, {}", sim.time(), sim.particles()[1].pos.len());
		sim.advance_with_callback(12.0, |s| {
			accumulate_density(&mut img, &s.particles()[1..2], scale, 8.0 * s.dt() as f32);
			accumulate_density(&mut img, &s.particles()[2..], scale, s.dt() as f32);
			Ok(())
		})?;
		save_density(&img, &dir.join(format!("density{:04}.png", i)))?;
		clear(&mut img);
	}

	Ok(())
}

fn clear(img: &mut Image<f32>) {
	//img.clear();
	for p in img.pixels_mut() {
		*p = 0.0;
	}
}

fn decay(img: &mut Image<f32>) {
	//img.clear();
	for p in img.pixels_mut() {
		*p *= 0.98;
	}
}

fn init_particles() -> Vec<Particle> {
	let mut particles = vec![
		Particle::new(1.0, vec2(0.0, 0.0), vec2(0.0, 0.0)),  // sun
		Particle::new(1e-3, vec2(1.0, 0.0), vec2(0.0, 1.0)), // jupiter
	];
	let rmin = 0.8;
	let rmax = 1.0;
	particles.append(&mut asteroids(NUM_ASTEROIDS, rmin, rmax));
	particles
}

fn asteroids(n: usize, rmin: f64, rmax: f64) -> Vec<Particle> {
	let mut particles = Vec::with_capacity(n);
	let rmin = rmin.powi(2);
	let rmax = rmax.powi(2);

	for i in 0..n {
		let dr = i as f64 / n as f64;
		let r = f64::sqrt(rmin + dr * (rmax - rmin));

		let theta = rnd(0.0, 2.0 * PI);
		let v = f64::sqrt(1.0 / r);
		let x = f64::cos(theta);
		let y = f64::sin(theta);
		let pos = r * vec2(x, y);
		let vel = v * vec2(-y, x);
		particles.push(Particle::new(ASTEROIDS_MASS, pos, vel));
	}
	particles
}

fn rnd(min: f64, max: f64) -> f64 {
	min + rand::random::<f64>() * (max - min)
}
