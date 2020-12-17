use super::error::*;
use super::result::*;

// Importing this trait adds a method `msg()` to all Result types,
// which prefixes Errors with an extra message. E.g.:
//
//	File::open(f).msg("read configuration")?
//
// returns the original error prefixed with "read configuration: ".
pub trait ErrorMessage<T> {
	fn msg(self, x: &str) -> Result<T>;
}

impl<T, E: std::error::Error> ErrorMessage<T> for std::result::Result<T, E> {
	fn msg(self, x: &str) -> Result<T> {
		match self {
			Ok(v) => Ok(v),
			Err(e) => Err(error(format!("{}: {}", x, e))),
		}
	}
}
