use super::prelude::*;

pub fn set_accel(particles: &[Particle], acc: &mut Vec<vec2>) {
	for i in 0..acc.len() {
		acc[i] = vec2::ZERO;
	}
	for (i, pi) in particles.iter().enumerate() {
		for j in (i + 1)..particles.len() {
			let pj = &particles[j];
			let delta = pj.pos - pi.pos;
			let len2 = delta.dot(delta);
			let len = len2.sqrt();
			let len3 = len2 * len;
			let acc_reduced = delta / len3;
			acc[i] += acc_reduced * pj.mass;
			acc[j] -= acc_reduced * pi.mass;
		}
	}
}

pub fn accel(particles: &[Particle]) -> Vec<vec2> {
	let mut acc = zeros(particles.len());
	set_accel(particles, &mut acc);
	acc
}

fn zeros(n: usize) -> Vec<vec2> {
	let mut dst = Vec::with_capacity(n);
	for _i in 0..n {
		dst.push(vec2::ZERO);
	}
	dst
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn pairwise() {
		let p = vec![
			Particle::new(2.0, vec2(-1.0, 0.0), vec2(0.0, 0.0)),
			Particle::new(1.0, vec2(1.0, 0.0), vec2(0.0, 0.0)),
		];

		let acc = accel(&p);
		assert_eq!(acc[0], vec2(0.25, 0.0));
		assert_eq!(acc[1], vec2(-0.5, 0.0));
	}
}
