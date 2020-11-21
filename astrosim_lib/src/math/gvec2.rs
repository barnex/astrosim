use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::*;

#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct gvec2<T: Copy> {
	pub x: T,
	pub y: T,
}

impl<T> PartialEq for gvec2<T>
where
	T: PartialEq + Copy,
{
	#[inline]
	fn eq(&self, rhs: &Self) -> bool {
		self.x == rhs.x && self.y == rhs.y
	}
}

impl<T: Copy> gvec2<T> {
	#[inline]
	pub fn new(x: T, y: T) -> Self {
		Self { x, y }
	}
}

impl<T> Add for gvec2<T>
where
	T: Add<T, Output = T> + Copy,
{
	type Output = Self;

	#[inline]
	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl<T> AddAssign for gvec2<T>
where
	T: AddAssign + Copy,
{
	#[inline]
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl<T> Div<T> for gvec2<T>
where
	T: Div<T, Output = T> + Copy,
{
	type Output = Self;

	#[inline]
	fn div(self, rhs: T) -> Self::Output {
		Self {
			x: self.x / rhs,
			y: self.y / rhs,
		}
	}
}

impl<T> Mul<T> for gvec2<T>
where
	T: Mul<T, Output = T> + Copy,
{
	type Output = Self;

	#[inline]
	fn mul(self, rhs: T) -> Self::Output {
		Self {
			x: self.x * rhs,
			y: self.y * rhs,
		}
	}
}

impl<T> MulAssign<T> for gvec2<T>
where
	T: MulAssign + Copy,
{
	#[inline]
	fn mul_assign(&mut self, rhs: T) {
		self.x *= rhs;
		self.y *= rhs;
	}
}

impl<T> Neg for gvec2<T>
where
	T: Neg<Output = T> + Copy,
{
	type Output = Self;

	#[inline]
	fn neg(self) -> Self::Output {
		Self {
			x: -self.x,
			y: -self.y,
		}
	}
}

impl<T> Sub for gvec2<T>
where
	T: Sub<T, Output = T> + Copy,
{
	type Output = Self;

	#[inline]
	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl<T> SubAssign for gvec2<T>
where
	T: SubAssign + Copy,
{
	#[inline]
	fn sub_assign(&mut self, rhs: Self) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}

impl<T> Display for gvec2<T>
where
	T: Copy + Display,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "({}, {})", self.x, self.y)
	}
}

impl<T> Debug for gvec2<T>
where
	T: Copy + Debug,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "({:?}, {:?})", self.x, self.y)
	}
}

impl<T> gvec2<T>
where
	T: Add<T, Output = T> + Mul<T, Output = T> + Sub<T, Output = T> + Copy,
{
	/// Dot (inner) product.
	#[inline]
	pub fn dot(self, rhs: gvec2<T>) -> T {
		self.x * rhs.x + self.y * rhs.y
	}

	/// Length squared (norm squared).
	#[inline]
	pub fn len2(self) -> T {
		self.dot(self)
	}
}
