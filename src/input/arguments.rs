use std::collections::HashSet;

use super::InputError;

#[derive(Debug, Clone)]
pub struct CliOption {
    long: &'static str,
    description: &'static str,
    short: Option<char>,
}

const OPTIONS: [CliOption; 4] = [
    CliOption {
        long: "ppm",
        description: "Export the image as a ppm",
        short: None,
    },
    CliOption {
        long: "progress-bar",
        description: "Display a progress bar while rendering",
        short: Some('p'),
    },
    CliOption {
        long: "help",
        description: "Print this help message",
        short: Some('h'),
    },
    CliOption {
        long: "version",
        description: "Print the version number",
        short: Some('v'),
    },
];

fn max_option_length() -> usize {
    OPTIONS
        .iter()
        .map(|opt| opt.long.len())
        .max()
        .expect("At least one option should exist")
}

fn name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

fn print_version() {
    println!("{} {}\n", name(), version());
}

fn print_help() {
    println!("{} {}\n", name(), version());
    println!("Usage: {} [OPTIONS] FILE\n", name());

    let maxlen = max_option_length();
    println!("Arguments:");
    for opt in OPTIONS {
        let short = if opt.short.is_some() { "-" } else { " " };
        let comma = if opt.short.is_some() { "," } else { " " };
        let length = maxlen - opt.long.len() + 2;
        println!(
            "  {}{}{} --{}{}{}",
            short,
            opt.short.unwrap_or(' '),
            comma,
            opt.long,
            " ".repeat(length),
            opt.description
        );
    }
}

/// Struct to hold configuration for the ray tracer
pub struct Config {
    /// file containing the scene
    input_file: String,
    options: HashSet<&'static str>,
}

impl Config {
    fn default() -> Config {
        Config {
            input_file: String::new(),
            options: HashSet::new(),
        }
    }

    /// Convert a message to a argument specific InputError
    fn parse_err(msg: &str) -> InputError {
        InputError::new(format!("Error while parsing Arguments:\n    {msg}"))
    }

    /// Build a config from a slice of Strings containing the arguments
    /// If this function returns Ok but with a None value, the program should exit early
    pub fn build(args: &[String]) -> Result<Option<Config>, InputError> {
        let mut config = Config::default();
        let mut unparsed = Vec::new();

        // skip first arg (the binary name)
        for arg in args.iter().skip(1) {
            if let Some(longopt) = arg.strip_prefix("--") {
                config.parse_longopt(longopt)?;
            } else if let Some(shortopt) = arg.strip_prefix("-") {
                config.parse_shortopt(shortopt)?;
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
    fn parse_longopt(&mut self, arg: &str) -> Result<(), InputError> {
        let opt = OPTIONS
            .iter()
            .find(|opt| opt.long == arg)
            .ok_or(Self::parse_err(&format!("Unknown long option '{arg}'")))?;

        self.options.insert(opt.long);
        Ok(())
    }

    /// Helper to parse (multiple) short options (prepended by '-')
    /// Each character is treated as it's own short option, so `-ph` is equal to `-p -h`
    fn parse_shortopt(&mut self, arg: &str) -> Result<(), InputError> {
        for c in arg.chars() {
            let opt = OPTIONS
                .iter()
                .find(|opt| opt.short.is_some_and(|o| o == c))
                .ok_or(Self::parse_err(&format!(
                    "Unknown short option{} '{arg}'",
                    if arg.len() > 1 { "s" } else { "" }
                )))?;

            self.options.insert(opt.long);
        }
        Ok(())
    }

    pub fn progress_bar(&self) -> bool {
        self.options.contains("progress-bar")
    }

    pub fn ppm(&self) -> bool {
        self.options.contains("ppm")
    }

    fn help(&self) -> bool {
        self.options.contains("help")
    }

    fn version(&self) -> bool {
        self.options.contains("version")
    }

    /// get a referencee to the provided input file path
    pub fn get_input(&self) -> &str {
        &self.input_file
    }
}
