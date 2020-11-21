mod dvec2;
mod fvec2;
mod gvec2;
mod util;

pub use util::*;

#[allow(non_camel_case_types)]
pub type vec2 = dvec2::dvec2;

#[inline]
pub fn vec2(x: f64, y: f64) -> vec2 {
	vec2 { x, y }
}
