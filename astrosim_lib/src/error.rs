use std::fmt;

pub type Error = Box<dyn std::error::Error>;

pub fn error(msg: String) -> Error {
	Box::new(ErrorMessage(msg))
}

#[derive(Debug)]
struct ErrorMessage(String);

impl fmt::Display for ErrorMessage {
	fn fmt(&self, w: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(w, "{}", &self.0)
	}
}

impl std::error::Error for ErrorMessage {}
