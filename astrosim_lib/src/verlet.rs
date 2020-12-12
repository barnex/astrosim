use super::prelude::*;

pub fn step(particles: &mut [Particle], acc: &[vec2], dt: f64) {
	for (i, p) in particles.iter_mut().enumerate() {
		p.vel = p.vel + acc[i] * dt;
		p.pos = p.pos + p.vel * dt;
	}
}
