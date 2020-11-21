use super::prelude::*;

pub struct Universe {
	particles: Vec<Particle>,
}

impl Universe {
	pub fn new() -> Self {
		Self {
			particles: Vec::new(),
		}
	}

	pub fn push(&mut self, p: Particle) {
		self.particles.push(p)
	}
}
