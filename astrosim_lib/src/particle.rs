pub use super::prelude::*;

#[derive(Clone, Debug)]
pub struct Particle {
	pub pos: vec2,
	pub vel: vec2,
	pub mass: f64,
}

impl Particle {
	pub fn new(mass: f64, pos: vec2, vel: vec2) -> Self {
		Self { mass, pos, vel }
	}
	pub fn random() -> Self {
		let mass = 1.0;
		let pos = vec2(2.0, 3.0);
		let vel = vec2(4.0, 5.0);
		Self { mass, pos, vel }
	}
}

// Add a constant velocity to each particle
// so that the system's net momentum becomes zero.
//
// This avoids that the group of particles drift out of the simulation frame.
//
// This does not otherwise alter the physics.
// It is merely equivalent to a moving simulation frame centered on
// the system's centre of gravity.
pub fn remove_net_momentum(particles: &mut [Particle]) {
	let mut total_mass = 0.0;
	let mut total_momentum = vec2(0.0, 0.0);
	for p in particles.iter_mut() {
		total_mass += p.mass;
		total_momentum += p.mass * p.vel;
	}
	let delta_v = total_momentum / total_mass;
	for p in particles {
		p.vel -= delta_v;
	}
}
