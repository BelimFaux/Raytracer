//! input module
//! responsible for receiving and parsing input files

mod arguments;
mod objparser;
mod serial_types;
mod xml;

use std::fmt::Display;

/// Struct for any kind of input error
/// (includes, commandline arguments, xml, and obj parsing)
#[derive(Debug)]
pub struct InputError(String);

impl InputError {
    pub fn new(msg: String) -> InputError {
        InputError(msg)
    }
}

impl Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

pub use arguments::Config;
pub use xml::*;
