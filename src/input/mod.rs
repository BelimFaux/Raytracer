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
pub struct InputError {
    title: String,
    msg: String,
}

impl InputError {
    #[must_use]
    pub fn new(title: String, msg: String) -> InputError {
        InputError { title, msg }
    }
}

const ERROR_COLOR: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

impl Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}:\n    {ERROR_COLOR}{}{RESET}",
            self.title, self.msg
        ))
    }
}

pub use arguments::Config;
pub use xml::*;
