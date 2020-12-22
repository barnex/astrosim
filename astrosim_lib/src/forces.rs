use super::prelude::*;

pub trait Forces {
	fn set_accel(&self, particles: &[Particle], acc: &mut [vec2]);

	fn accel(&self, particles: &[Particle]) -> Vec<vec2> {
		let mut acc = zeros(particles.len());
		self.set_accel(particles, &mut acc);
		acc
	}
}
