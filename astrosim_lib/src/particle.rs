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
