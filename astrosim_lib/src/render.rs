use super::image::Image;
use super::prelude::*;
use std::path::Path;

pub fn save_density<P: AsRef<Path>>(density: &Image<f32>, file: P) -> Result<()> {
	let max = density.pixels().iter().fold(0.0, |a, b| f32::max(a, *b));
	let (w, h) = density.dimensions();
	let img = image::ImageBuffer::from_fn(w as u32, h as u32, |x, y| {
		let density = density[y as usize][x as usize];
		let v = ((density / max).sqrt() * 255.0) as u8;
		image::Rgba([v, v, v, 255])
	});
	img.save(file)?;
	Ok(())
}

pub fn render_density(particles: &[Particle], npix: u32, scale: f64) -> Image<f32> {
	let mut img = Image::new(npix, npix);

	for p in particles {
		let screen_pos = (p.pos / (2.0 * scale) + vec2(0.5, 0.5)) * (npix as f64) + vec2(0.5, 0.5);
		let x = screen_pos.x as i32;
		let y = screen_pos.y as i32;
		let npix = npix as i32;
		if x >= 0 && x < npix && y >= 0 && y < npix {
			img[y as usize][x as usize] += 1.0;
		}
	}
	img
}

pub fn accumulate_density(img: &mut Image<f32>, particles: &[Particle], scale: f64, weight: f32) {
	let npix = img.width();
	for p in particles {
		let screen_pos = (p.pos / (2.0 * scale) + vec2(0.5, 0.5)) * (npix as f64) + vec2(0.5, 0.5);
		let x = screen_pos.x as i32;
		let y = screen_pos.y as i32;
		let npix = npix as i32;
		if x >= 0 && x < npix && y >= 0 && y < npix {
			img[y as usize][x as usize] += weight; // TODO: mass? What about 0 mass particles?
		}
	}
}
