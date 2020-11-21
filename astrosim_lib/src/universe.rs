use super::prelude::*;

pub struct Universe {
	particles: Vec<Particle>,
	accel: Vec<vec2>,
}

impl Universe {
	pub fn new() -> Self {
		Self {
			particles: Vec::new(),
			accel: Vec::new(),
		}
	}

	pub fn push(&mut self, p: Particle) {
		self.particles.push(p);
		self.accel.push(vec2::ZERO);
	}

	#[must_use]
	pub fn accel(&self) -> Vec<vec2> {
		let mut dst = zeros(self.particles.len());
		self.set_accel(&mut dst);
		dst
	}

	pub fn set_accel(&self, dst: &mut Vec<vec2>) {
		let particles = &self.particles;
		for (i, p) in particles.iter().enumerate() {
			for j in (i + 1)..particles.len() {
				let q = &particles[j];
				let delta = p.pos - q.pos;
				let len2 = delta.dot(delta);
				let len = len2.sqrt();
				let len3 = len2 * len;
				let acc_reduced = delta / len3;
				let acc_q = acc_reduced * p.mass;
				let acc_p = -acc_reduced * q.mass;
				dst[i] += acc_p;
				dst[j] += acc_q;
			}
		}
	}
}

fn zeros(n: usize) -> Vec<vec2> {
	let mut dst = Vec::with_capacity(n);
	for _i in 0..n {
		dst.push(vec2::ZERO);
	}
	dst
}
