use astrosim_lib::bruteforce;
use astrosim_lib::prelude::*;
use std::time::Instant;

fn main() {
	for exp in 1..15 {
		let n = usize::pow(2, exp);
		benchmark(n);
	}
}

fn benchmark(n: usize) {
	let particles = random_particles(n);

	let mut acc = zeros(n);

	let start = Instant::now();
	bruteforce::set_accel(&particles, &mut acc);

	let duration = start.elapsed();

	let n = n as f64;
	let ms = duration.as_secs_f64() * 1000.0;
	let ns = duration.as_secs_f64() * 1000000.0;
	println!(
		"n: {}: {} ms = {} ns/particle",
		n,
		ms as f32,
		(ns / n) as f32
	);
}

fn random_particles(n: usize) -> Vec<Particle> {
	let mut particles = Vec::with_capacity(n);
	for _i in 0..n {
		particles.push(Particle::random());
	}
	particles
}
