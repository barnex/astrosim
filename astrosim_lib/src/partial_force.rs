pub use super::prelude::*;

pub struct PartialForce {
	cutoff_index: usize,
}

impl PartialForce {
	pub fn new(cutoff_index: usize) -> Self {
		PartialForce { cutoff_index }
	}
}

impl Forces for PartialForce {
	fn set_accel(&self, particles: &[Particle], acc: &mut [vec2]) {
		debug_assert!(particles.len() == acc.len());

		for i in 0..acc.len() {
			acc[i] = vec2::ZERO;
		}
		for (i, pi) in particles[..self.cutoff_index].iter().enumerate() {
			let mut acci = vec2::ZERO;
			for j in (i + 1)..particles.len() {
				let pj = &particles[j];
				let delta = pj.pos - pi.pos;
				let len2 = delta.dot(delta);
				let len = len2.sqrt();
				let len3 = len2 * len;
				let acc_reduced = delta / len3;
				acci += acc_reduced * pj.mass;
				acc[j] -= acc_reduced * pi.mass;
			}
			acc[i] += acci;
		}
	}
}
