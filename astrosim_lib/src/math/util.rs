use super::*;

pub fn zeros(n: usize) -> Vec<vec2> {
	let mut dst = Vec::with_capacity(n);
	for _i in 0..n {
		dst.push(vec2::ZERO);
	}
	dst
}
