use super::prelude::*;

pub struct BruteForce();

impl Forces for BruteForce {
	fn set_accel(&self, particles: &[Particle], acc: &mut [vec2]) {
		debug_assert!(particles.len() == acc.len());

		for i in 0..acc.len() {
			acc[i] = vec2::ZERO;
		}
		for (i, pi) in particles.iter().enumerate() {
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

impl BruteForce {
	pub fn new() -> Self {
		BruteForce()
	}
}

#[cfg(test)]
mod test {
	use super::*;

	// Test the force between a pair of particles along the x axis
	//
	//   *-----*
	//
	#[test]
	fn pairwise_x() {
		let p = vec![Particle::new(2.0, vec2(-1.0, 0.0), vec2(0.0, 0.0)), Particle::new(1.0, vec2(1.0, 0.0), vec2(0.0, 0.0))];

		let acc = (BruteForce()).accel(&p);
		assert_eq!(acc[0], vec2(0.25, 0.0));
		assert_eq!(acc[1], vec2(-0.5, 0.0));
	}

	// Test the force between a pair of particles along the y axis
	//    *
	//    |
	//    *
	#[test]
	fn pairwise_y() {
		let p = vec![Particle::new(2.0, vec2(0.0, -1.0), vec2(0.0, 0.0)), Particle::new(1.0, vec2(0.0, 1.0), vec2(0.0, 0.0))];

		let acc = (BruteForce()).accel(&p);
		assert_eq!(acc[0], vec2(0.0, 0.25));
		assert_eq!(acc[1], vec2(0.0, -0.5));
	}

	// Test the force between 3 particles.
	//    *
	//   / \
	//  *---*
	#[test]
	fn treebody() {
		let p = vec![
			Particle::new(1.0, vec2(-1.0, 0.0), vec2(0.0, 0.0)),
			Particle::new(1.0, vec2(1.0, 0.0), vec2(0.0, 0.0)),
			Particle::new(2.0, vec2(0.0, 0.5), vec2(0.0, 0.0)),
		];

		let acc = (BruteForce()).accel(&p);
		assert_eq!(acc[0], vec2(1.6810835055998654, 0.7155417527999327));
		assert_eq!(acc[1], vec2(-1.6810835055998654, 0.7155417527999327));
		assert_eq!(acc[2], vec2(0.0, -0.7155417527999327));
	}
}
