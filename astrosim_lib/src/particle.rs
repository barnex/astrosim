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
}
