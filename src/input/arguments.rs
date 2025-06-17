use super::InputError;

/// Struct to hold configuration for the ray tracer
pub struct Config {
    /// file containing the scene
    input_file: String,
}

impl Config {
    /// build a config from a slice of Strings containing the arguments
    pub fn build(args: &[String]) -> Result<Config, InputError> {
        if args.len() < 2 {
            Err(InputError(String::from("Missing input path")))
        } else {
            Ok(Config {
                input_file: args[1].clone(),
            })
        }
    }

    /// get a referencee to the provided input file path
    pub fn get_input(&self) -> &str {
        &self.input_file
    }
}
