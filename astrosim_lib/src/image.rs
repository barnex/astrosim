use std::ops::{Index, IndexMut};

/// 2D rectangular array of generic values.
#[derive(Debug, PartialEq, Clone)]
pub struct Image<C> {
	dim: (usize, usize),
	values: Vec<C>,
}

impl<'a, C> Image<C>
where
	C: Copy + Default,
{
	/// new constructs an Image with given width and height.
	pub fn new(w: u32, h: u32) -> Image<C> {
		Image {
			dim: (w as usize, h as usize),
			values: vec![C::default(); w as usize * h as usize],
		}
	}

	//pub fn from_fn<F: Fn(i32, i32) -> C>((w, h): (i32, i32), f: F) -> Self {
	//	let mut img = Self::new(w, h);
	//	for iy in 0..(h as usize) {
	//		for ix in 0..(w as usize) {
	//			img[iy][ix] = f(ix as i32, iy as i32);
	//		}
	//	}
	//	img
	//}

	pub fn at(&self, p: (i32, i32)) -> C {
		self[p.1 as usize][p.0 as usize]
	}

	/// width of the image, in pixels
	pub fn width(&self) -> usize {
		self.dim.0
	}

	/// height of the image, in pixels
	pub fn height(&self) -> usize {
		self.dim.1
	}

	/// width and height of the image
	pub fn dimensions(&self) -> (i32, i32) {
		(self.dim.0 as i32, self.dim.1 as i32)
	}

	/// pixels in row-major order, iterable.
	pub fn pixels(&self) -> &[C] {
		&self.values
	}

	/// pixels in row-major order, iterable.
	pub fn pixels_mut(&mut self) -> &mut [C] {
		&mut self.values
	}
}

impl<C> Default for Image<C>
where
	C: Copy + Default,
{
	fn default() -> Self {
		Self {
			dim: (0, 0),
			values: Vec::new(),
		}
	}
}

impl<C> Index<usize> for Image<C>
where
	C: Copy + Default,
{
	type Output = [C];

	fn index(&self, i: usize) -> &[C] {
		let l = i * self.width();
		let h = l + self.width();
		&self.values[l..h]
	}
}

impl<C> IndexMut<usize> for Image<C>
where
	C: Copy + Default,
{
	fn index_mut(&mut self, i: usize) -> &mut [C] {
		let l = i * self.width();
		let h = l + self.width();
		&mut self.values[l..h]
	}
}

impl Image<f32> {
	pub fn raw_bgra(&self, scale: f32) -> Vec<u8> {
		let (w, h) = self.dimensions();
		let mut raw = Vec::with_capacity((w * h * 4) as usize);
		for iy in 0..h {
			for ix in 0..w {
				let value = self[iy as usize][ix as usize];
				let color = i32::max(255, (value * scale) as i32) as u8;
				raw.push(color);
				raw.push(color);
				raw.push(color);
				raw.push(255);
			}
		}
		raw
	}
}
