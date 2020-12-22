extern crate rand;
use astrosim_lib::prelude::*;
use std::fs;
use std::path::PathBuf;

const NUM_ASTEROIDS: usize = 2000;

fn main() -> Result<()> {
	let dir = PathBuf::from("kirkwood_gaps_aliased.out");
	fs::create_dir_all(&dir)?;

	let particles = init_particles();
	let mut sim = Stepper::new(particles, BruteForce::new());
	sim.target_error = 0.0001;
	sim.min_dt = 0.000002;

	let (w, h) = (512, 512);
	let mut img = Image::<f32>::new(w, h);
	//let mut overall = Image::<f32>::new(w, h);
	let scale = 1.5;

	let mut delta = 0.005;
	//let delta = 2.0 * PI;
	//let mut py = sim.particles()[1].pos.y;

	for i in 0..10000 {
		println!("i: {}, t: {}, dt: {}", i, sim.time(), sim.dt);
		delta *= 1.005;
		if delta > 4.0 * PI {
			delta = 4.0 * PI;
		}
		sim.advance_with_callback(delta, |s| {
			accumulate_density(&mut img, &s.particles()[2..], scale, s.dt() as f32);
			accumulate_density(&mut img, &s.particles()[0..1], scale, 0.3 * s.dt() as f32);
			accumulate_density(&mut img, &s.particles()[1..2], scale, 0.3 * s.dt() as f32);
			// let new_py = s.particles()[1].pos.y;
			// if py > 0.0 && new_py < 0.0 {
			// 	accumulate_density(&mut img, &s.particles()[0..1], scale, 0.01);
			// 	accumulate_density(&mut img, &s.particles()[1..2], scale, 0.01);
			// 	accumulate_density(&mut img, &s.particles()[2..], scale, 1.0);
			// 	save_density(&img, &dir.join(format!("density{:05}.png", i)))?;
			// 	decay(&mut img);
			// }
			// py = new_py;
			Ok(())
		})?;
	}

	//for i in 0..1000 {
	//	println!("i: {}, dt: {}", i, sim.dt);
	//	let delta = 2.0 * PI;
	//	sim.advance_with_callback(delta, |s| Ok(accumulate_density(&mut img, &s.particles()[1..], scale, s.dt() as f32)))?;

	//	save_density(&img, &dir.join(format!("density{:04}.jpg", i)))?;
	//	clear(&mut img);
	//}

	Ok(())
}

//fn clear(img: &mut Image<f32>) {
//	for p in img.pixels_mut() {
//		*p = 0.0;
//	}
//}

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
	particles.append(&mut asteroids(NUM_ASTEROIDS, 0.22, 0.99));
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
		let mass = 0.0;
		particles.push(Particle::new(mass, pos, vel));
	}
	particles
}

fn rnd(min: f64, max: f64) -> f64 {
	min + rand::random::<f64>() * (max - min)
}
