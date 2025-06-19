use std::collections::HashSet;

use super::InputError;

#[derive(Debug, Clone)]
pub struct CliOption {
    long: &'static str,
    description: &'static str,
    short: Option<char>,
}

const OPTIONS: [CliOption; 3] = [
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

pub fn print_help() {
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

    /// build a config from a slice of Strings containing the arguments
    pub fn build(args: &[String]) -> Result<Config, InputError> {
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
            return Ok(config);
        }

        let file = unparsed
            .first()
            .ok_or(InputError(String::from("Missing input path")))?;

        config.input_file = file.to_string();

        Ok(config)
    }

    fn parse_longopt(&mut self, arg: &str) -> Result<(), InputError> {
        let opt = OPTIONS
            .iter()
            .find(|opt| opt.long == arg)
            .ok_or(InputError(format!("Unknown long option '{arg}'")))?;

        self.options.insert(opt.long);
        Ok(())
    }

    fn parse_shortopt(&mut self, arg: &str) -> Result<(), InputError> {
        for c in arg.chars() {
            let opt = OPTIONS
                .iter()
                .find(|opt| opt.short.is_some_and(|o| o == c))
                .ok_or(InputError(format!(
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

    pub fn help(&self) -> bool {
        self.options.contains("help")
    }

    /// get a referencee to the provided input file path
    pub fn get_input(&self) -> &str {
        &self.input_file
    }
}
