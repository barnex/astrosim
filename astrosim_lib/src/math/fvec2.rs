use super::gvec2::*;
use std::ops::*;

#[allow(non_camel_case_types)]
pub type fvec2 = gvec2<f32>;

pub fn fvec2(x: f32, y: f32) -> fvec2 {
	fvec2 { x, y }
}

impl Mul<fvec2> for f32 {
	type Output = fvec2;

	#[inline]
	fn mul(self, rhs: fvec2) -> Self::Output {
		rhs.mul(self)
	}
}

impl fvec2 {
	/// Length (norm).
	#[inline]
	pub fn len(self) -> f32 {
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
	pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

	/// Unit vector along X.
	pub const EX: Self = Self { x: 1.0, y: 0.0 };

	/// Unit vector along Y.
	pub const EY: Self = Self { x: 0.0, y: 1.0 };
}
