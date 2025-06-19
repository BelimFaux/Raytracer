//! input module
//! responsible for receiving and parsing input files

mod arguments;
mod objparser;
mod serial_types;
mod xml;

#[derive(Debug)]
pub struct InputError(String);

impl Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

use std::fmt::Display;

pub use arguments::Config;
pub use xml::*;
