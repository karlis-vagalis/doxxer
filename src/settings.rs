use clap::ValueEnum;
use regex::Regex;
use std::path::PathBuf;

use crate::{config::Configuration, Cli, Format, Strategy};

pub mod default {
    pub static DIRECTORY: &str = ".";
    pub static TAG_FILTER: &str = "";
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
    pub tag_filter: Regex,
    pub output_template: String,
    pub output_format: Format
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            directory: PathBuf::from(default::DIRECTORY),
            tag_filter: Regex::new(default::TAG_FILTER).unwrap(),
            output_template: default::OUTPUT_TEMPLATE.to_string(),
            output_format: Format::Plain,
            config: Configuration::load(None),
        }
    }
}
impl From<&PathBuf> for Settings {
    fn from(config_path: &PathBuf) -> Self {
        Self {
            directory: PathBuf::from(default::DIRECTORY),
            tag_filter: Regex::new(default::TAG_FILTER).unwrap(),
            output_template: default::OUTPUT_TEMPLATE.to_string(),
            output_format: Format::Plain,
            config: Configuration::load(Some(config_path)),
        }
    }
}
impl Settings {
    /// Discovers/loads configuration from specified path
    pub fn apply(&mut self, args: &Cli) {
        let command = match &args.cmd {
            crate::Commands::Current { .. } => "current",
            crate::Commands::Next { strategy, .. } => match strategy {
                Some(Strategy::Major { .. }) => "next.major",
                Some(Strategy::Minor { .. }) => "next.minor",
                Some(Strategy::Patch { .. }) => "next.patch",
                Some(Strategy::Prerelease { .. }) => "next.prerelease",
                Some(Strategy::PreMajor { .. }) => "next.pre-major",
                Some(Strategy::PreMinor { .. }) => "next.pre-minor",
                Some(Strategy::PrePatch { .. }) => "next.pre-patch",
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
        if let Ok(filter) = self.config.get::<String>(command, "tag_filter") {
            self.tag_filter = Regex::new(&filter).unwrap();
        }
        if let Some(filter) = &args.filter_options.tag_filter {
            self.tag_filter = Regex::new(filter).unwrap();
        };

        // Output template
        if let Ok(template) = self.config.get::<String>(command, "output_template") {
            self.output_template = template
        }
        if let Some(template) = &args.output_options.output_template {
            self.output_template = template.clone();
        };

        // Output format
        if let Ok(format) = self.config.get::<String>(command, "output_format") {
            self.output_format = Format::from_str(format.as_str(), true).unwrap();
        }
        if let Some(format) = &args.output_options.format {
            self.output_format = format.clone();
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
