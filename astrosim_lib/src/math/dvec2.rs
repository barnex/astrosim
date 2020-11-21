use super::gvec2::*;
use std::ops::*;

#[allow(non_camel_case_types)]
pub type dvec2 = gvec2<f64>;

pub fn dvec2(x: f64, y: f64) -> dvec2 {
	dvec2 { x, y }
}

impl Mul<dvec2> for f64 {
	type Output = dvec2;

	#[inline]
	fn mul(self, rhs: dvec2) -> Self::Output {
		rhs.mul(self)
	}
}

impl dvec2 {
	/// Length (norm).
	#[inline]
	pub fn len(self) -> f64 {
		self.len2().sqrt()
	}

	/// Returns a vector with the same direction but unit length.
	#[inline]
	#[must_use]
	pub fn normalized(self) -> Self {
		self * (1.0 / self.len())
	}

	/// Re-scale the vector to unit length.
	#[inline]
	pub fn normalize(&mut self) {
		*self = self.normalized()
	}

	pub fn is_finite(&self) -> bool {
		self.x.is_finite() && self.y.is_finite()
	}

	/// The zero vector.
	pub const ZERO: dvec2 = Self { x: 0.0, y: 0.0 };

	/// Unit vector along X.
	pub const EX: dvec2 = Self { x: 1.0, y: 0.0 };

	/// Unit vector along Y.
	pub const EY: dvec2 = Self { x: 0.0, y: 1.0 };
}
