use super::image::Image;
use super::prelude::*;

pub fn render(particles: &[Particle], npix: u32, scale: f64) -> Image<f32> {
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
