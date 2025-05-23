use regex::Regex;
use std::path::PathBuf;

use crate::{config::Configuration, Cli, Strategy};

pub mod default {
    pub static DIRECTORY: &str = ".";
    pub static FILTER: &str = "";
    pub static OUTPUT_TEMPLATE: &str = "{version}";
    pub static PRERELEASE_TEMPLATE: &str = "{identifier}.{inc}";
    pub static DEV_TEMPLATE: &str = "{pre}.{identifier}.{distance}";
    pub static BUILD_TEMPLATE: &str = "{hash}";
    pub static PRERELEASE_IDENTIFIER: &str = "build";
    pub static DEV_IDENTIFIER: &str = "dev";
    pub static INCREMENT: u64 = 1;
    pub static CONFIG_FILE_NAME: &str = "doxxer";
}

#[derive(Debug)]
pub struct Settings {
    config: Configuration,
    pub directory: PathBuf,
    pub filter: Regex,
    pub output_template: String,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            directory: PathBuf::from(default::DIRECTORY),
            filter: Regex::new(default::FILTER).unwrap(),
            output_template: default::OUTPUT_TEMPLATE.to_string(),
            config: Configuration::load(None),
        }
    }
}
impl From<&PathBuf> for Settings {
    fn from(config_path: &PathBuf) -> Self {
        Self {
            directory: PathBuf::from(default::DIRECTORY),
            filter: Regex::new(default::FILTER).unwrap(),
            output_template: default::OUTPUT_TEMPLATE.to_string(),
            config: Configuration::load(Some(config_path)),
        }
    }
}
impl Settings {
    /// Discovers/loads configuration from specified path
    pub fn apply(&mut self, args: &Cli) {
        let command = match &args.cmd {
            crate::Commands::Current { field: _ } => "current",
            crate::Commands::Next { strategy, field: _ } => match strategy {
                Some(strategy) => match strategy {
                    Strategy::Major { bump_options: _ } => "next.major",
                    Strategy::Minor { bump_options: _ } => "next.minor",
                    Strategy::Patch { bump_options: _ } => "next.patch",
                    Strategy::Prerelease {
                        prerelease_options: _,
                    } => "next.prerelease",
                    Strategy::PreMajor {
                        prerelease_options: _,
                        bump_options: _,
                    } => "next.pre-major",
                    Strategy::PreMinor {
                        prerelease_options: _,
                        bump_options: _,
                    } => "next.pre-minor",
                    Strategy::PrePatch {
                        prerelease_options: _,
                        bump_options: _,
                    } => "next.pre-patch",
                },
                None => "next",
            },
        };

        // Dir
        if let Ok(dir) = self.config.get::<String>(command, "directory") {
            self.directory = PathBuf::from(dir);
        }
        if let Some(directory) = &args.directory {
            self.directory = directory.clone();
        };

        // Filter
        if let Ok(filter) = self.config.get::<String>(command, "filter") {
            self.filter = Regex::new(&filter).unwrap();
        }
        if let Some(filter) = &args.filter_options.filter {
            self.filter = Regex::new(filter).unwrap();
        };

        // Output template
        if let Ok(template) = self.config.get::<String>(command, "output_template") {
            self.output_template = template
        }
        if let Some(template) = &args.output_options.output_template {
            self.output_template = template.clone();
        };

        // Convert path to the absolute path
        self.directory = std::path::absolute(&self.directory).unwrap();
    }

    pub fn validate(&self) {
        if !self.output_template.contains("{version}") {
            eprintln!(
                "Output template \"{}\" is missing required variable {{version}}",
                self.output_template
            );
            std::process::exit(1);
        }
    }
}
