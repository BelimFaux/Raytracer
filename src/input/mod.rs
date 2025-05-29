mod arguments;
mod serial_types;
mod xml;

pub struct InputError(String);

impl Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

use std::fmt::Display;

pub use arguments::Config;
pub use xml::*;
