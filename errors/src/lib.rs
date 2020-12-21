//! Utilities for error handling:
//!
//!  * Type `Error`, representing "any error".
//!  * A corresponding `Result` type.
//!  * The `ErrorMessage` trait, adding a method `msg(...)` to Results.
//!
mod error;
mod errormessage;
mod result;

pub use error::*;
pub use errormessage::*;
pub use result::*;
