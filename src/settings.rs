use std::path::PathBuf;

use clap::ValueEnum as _;
use regex::Regex;

use crate::{cli::{Cli, Format, Strategy}, config::Configuration, default};

pub struct FilterSettings {
    pub tag: Regex
}

pub struct OutputSettings {
    pub format: Format,
    pub template: String
}

pub struct Settings {
    pub directory: PathBuf,
    pub filter: FilterSettings,
    pub output: OutputSettings
}

impl Settings {
    pub fn merge (cli: &Cli, config: &Configuration) -> Self {
        let command = match &cli.cmd {
            crate::Commands::Current { .. } => "current",
            crate::Commands::Next { strategy, .. } => match strategy {
                Some(Strategy::Major { .. }) => "next.major",
                Some(Strategy::Minor { .. }) => "next.minor",
                Some(Strategy::Patch { .. }) => "next.patch",
                Some(Strategy::Prerelease { .. }) => "next.prerelease",
                Some(Strategy::PreMajor { .. }) => "next.pre-major",
                Some(Strategy::PreMinor { .. }) => "next.pre-minor",
                Some(Strategy::PrePatch { .. }) => "next.pre-patch",
                Some(Strategy::Dev { .. }) => "next.dev",
                None => "next",
            },
        };

        let directory = match &cli.directory {
            Some(directory) => directory.clone(),
            None => match config.get::<String>(command, "directory") {
                Ok(dir) => PathBuf::from(dir),
                Err(_) => PathBuf::from(default::DIRECTORY),
            },
        };

        let filter_tag = match &cli.filter.tag {
            Some(filter) => filter.clone(),
            None => match config.get::<String>(command, "filter.tag") {
                Ok(tag_filter) => tag_filter,
                Err(_) => default::TAG_FILTER.to_string(),
            },
        };
        let filter_tag = Regex::new(&filter_tag).unwrap();

        let output_format = match &cli.output.format {
            Some(format) => format.clone(),
            None => match config.get::<String>(command, "output.format") {
                Ok(format) => Format::from_str(&format, true).unwrap(),
                Err(_) => Format::Plain,
            },
        };

        let output_template = match &cli.output.template {
            Some(template) => template.clone(),
            None => match config.get::<String>(command, "output.template") {
                Ok(template) => template,
                Err(_) => default::OUTPUT_TEMPLATE.to_string(),
            },
        };

        Self {
            directory,
            filter: FilterSettings { tag: filter_tag },
            output: OutputSettings { format: output_format, template: output_template}
        }
    }

    pub fn validate(&self) {
        
        if !self.output.template.contains("{version}") {
            eprintln!(
                "Output template \"{}\" is missing required variable {{version}}",
                self.output.template
            );
            std::process::exit(1);
        }
        
    }
}