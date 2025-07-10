use std::collections::HashMap;

use super::InputError;

#[derive(Debug, Clone)]
enum OptAction {
    Toggle,
    Set {
        default: &'static str,
        placeholder: &'static str,
    },
}

#[derive(Debug, Clone)]
struct CliOption {
    long: &'static str,
    description: &'static str,
    short: Option<char>,
    action: OptAction,
}

/// All cli options that should be parsed
const OPTIONS: [CliOption; 6] = [
    CliOption {
        long: "ppm",
        description: "Export the image as a ppm",
        short: None,
        action: OptAction::Toggle,
    },
    CliOption {
        long: "progress-bar",
        description: "Display a progress bar while rendering",
        short: Some('p'),
        action: OptAction::Toggle,
    },
    CliOption {
        long: "blur",
        description: "Instead of an animation, render movement as blur",
        short: None,
        action: OptAction::Toggle,
    },
    CliOption {
        long: "outdir",
        description: "Set the directory to save the image to",
        short: Some('o'),
        action: OptAction::Set {
            default: "output",
            placeholder: "<DIR>",
        },
    },
    CliOption {
        long: "help",
        description: "Print this help message",
        short: Some('h'),
        action: OptAction::Toggle,
    },
    CliOption {
        long: "version",
        description: "Print the version number",
        short: Some('V'),
        action: OptAction::Toggle,
    },
];

/// return the maximum length of long name + default value
fn max_option_length() -> usize {
    OPTIONS
        .iter()
        .map(|opt| match opt.action {
            OptAction::Toggle => opt.long.len(),
            OptAction::Set {
                default: _,
                placeholder,
            } => opt.long.len() + placeholder.len(),
        })
        .max()
        .expect("At least one option should exist")
}

fn name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// print version of the program
fn print_version() {
    println!("{} {}\n", name(), version());
}

/// print help text for the program
fn print_help() {
    println!("{} {}\n", name(), version());
    println!("Usage: {} [OPTIONS] FILE\n", name());

    let maxlen = max_option_length();
    println!("Arguments:");
    for opt in OPTIONS {
        let short = if opt.short.is_some() { "-" } else { " " };
        let comma = if opt.short.is_some() { "," } else { " " };
        let (default, placeholder) = match opt.action {
            OptAction::Set {
                default,
                placeholder,
            } => (format!("(default: '{default}')"), placeholder),
            _ => ("".to_string(), ""),
        };
        let length = maxlen - opt.long.len() + 2 - placeholder.len();
        println!(
            "  {}{}{} --{} {}{}{} {}",
            short,
            opt.short.unwrap_or(' '),
            comma,
            opt.long,
            placeholder,
            " ".repeat(length),
            opt.description,
            default
        );
    }
}

/// Struct to hold configuration for the ray tracer
#[derive(Debug)]
pub struct Config {
    /// file containing the scene
    input_file: String,
    options: HashMap<&'static str, String>,
}

impl Config {
    fn default() -> Config {
        let options: HashMap<_, _> = OPTIONS
            .iter()
            .filter_map(|opt| match opt.action {
                OptAction::Set { default, .. } => Some((opt.long, default.to_string())),
                OptAction::Toggle => None,
            })
            .collect();

        Config {
            input_file: String::new(),
            options,
        }
    }

    /// Convert a message to a argument specific InputError
    fn parse_err(msg: &str) -> InputError {
        InputError::new("Error while parsing Arguments".to_string(), msg.to_string())
    }

    fn insert_options<'a, I>(&mut self, opt: &CliOption, iter: &mut I) -> Result<(), InputError>
    where
        I: Iterator<Item = &'a String>,
    {
        match opt.action {
            OptAction::Toggle => self.options.insert(opt.long, String::from("")),
            OptAction::Set { .. } => self.options.insert(
                opt.long,
                iter.next()
                    .ok_or(Self::parse_err(&format!(
                        "Expected value for option {}",
                        opt.long,
                    )))?
                    .clone(),
            ),
        };
        Ok(())
    }

    /// Build a config from a slice of Strings containing the arguments
    /// If this function returns Ok but with a None value, the program should exit early
    pub fn build(args: &[String]) -> Result<Option<Config>, InputError> {
        let mut config = Config::default();
        let mut unparsed = Vec::new();

        // skip first arg (the binary name)
        let mut iter = args.iter().skip(1);
        while let Some(arg) = iter.next() {
            if let Some(longopt) = arg.strip_prefix("--") {
                let opt = Config::parse_longopt(longopt)?;
                config.insert_options(opt, &mut iter)?;
            } else if let Some(shortopt) = arg.strip_prefix("-") {
                let opts = Config::parse_shortopt(shortopt)?;

                for opt in opts {
                    config.insert_options(opt, &mut iter)?;
                }
            } else {
                unparsed.push(arg);
            }
        }

        if config.help() {
            print_help();
            return Ok(None);
        }

        if config.version() {
            print_version();
            return Ok(None);
        }

        let file = unparsed
            .first()
            .ok_or(Self::parse_err("Missing input path"))?;

        config.input_file = file.to_string();

        Ok(Some(config))
    }

    /// Helper to parse a long option (prepended by '--')
    fn parse_longopt(arg: &str) -> Result<&CliOption, InputError> {
        OPTIONS
            .iter()
            .find(|opt| opt.long == arg)
            .ok_or(Self::parse_err(&format!("Unknown long option '{arg}'")))
    }

    /// Helper to parse (multiple) short options (prepended by '-')
    /// Each character is treated as it's own short option, so `-ph` is equal to `-p -h`
    fn parse_shortopt(arg: &str) -> Result<Vec<&CliOption>, InputError> {
        arg.chars()
            .map(|c| {
                OPTIONS
                    .iter()
                    .find(|opt| opt.short.is_some_and(|o| o == c))
                    .ok_or(Self::parse_err(&format!(
                        "Unknown short option{} '{arg}'",
                        if arg.len() > 1 { "s" } else { "" }
                    )))
            })
            .collect()
    }

    pub fn progress_bar(&self) -> bool {
        self.options.contains_key("progress-bar")
    }

    pub fn ppm(&self) -> bool {
        self.options.contains_key("ppm")
    }

    pub fn blur(&self) -> bool {
        self.options.contains_key("blur")
    }

    pub fn outdir(&self) -> &str {
        self.options
            .get("outdir")
            .expect("outdir should always be inside")
    }

    fn help(&self) -> bool {
        self.options.contains_key("help")
    }

    fn version(&self) -> bool {
        self.options.contains_key("version")
    }

    /// get a referencee to the provided input file path
    pub fn get_input(&self) -> &str {
        &self.input_file
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_args() {
        let args = &[
            "test".to_string(),
            "input.obj".to_string(),
            "--outdir".to_string(),
            "output".to_string(),
            "--ppm".to_string(),
            "--progress-bar".to_string(),
        ];

        let config = Config::build(args).unwrap().unwrap();

        assert_eq!(config.get_input(), "input.obj");
        assert_eq!(config.outdir(), "output");
        assert!(config.ppm());
        assert!(config.progress_bar());
    }

    #[test]
    fn help_version_early_exit() {
        let args = &["test".to_string(), "--help".to_string()];
        let config = Config::build(args).unwrap();
        assert!(config.is_none());

        let args = &["test".to_string(), "--version".to_string()];
        let config = Config::build(args).unwrap();
        assert!(config.is_none());
    }
}
