use regex::Regex;
use std::path::PathBuf;

use config::{Config};

use crate::{config::Configuration, Cli};

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
        // Override with CLI options
        if let Some(directory) = &args.directory {
            self.directory = directory.clone();
        };
        if let Some(filter) = &args.filter_options.filter {
            self.filter = Regex::new(filter).unwrap();
        };
        if let Some(template) = &args.output_options.output_template {
            self.output_template = template.clone();
        };
        //dbg!(&self);

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
