use std::path::PathBuf;

use clap::ValueEnum as _;
use regex::Regex;

use crate::{
    cli::{
        BuildMetadataOptions, BumpingOptions, Cli, Format, PreReleaseWithBumpArgs, PrereleaseArgs,
        PrereleaseOptions, StandardBumpArgs, Strategy,
    },
    config::Configuration,
    default,
};
#[derive(Debug)]
pub struct FilterSettings {
    pub tag: Regex,
}
#[derive(Debug)]
pub struct OutputSettings {
    pub format: Format,
    pub template: String,
}
#[derive(Debug)]
pub struct BumpSettings {
    pub increment: u64,
}
#[derive(Debug)]
pub struct PrereleaseSettings {
    pub identifier: String,
    pub template: String,
}
#[derive(Debug)]
pub struct BuildMetadataSettings {
    pub template: String,
}

#[derive(Debug)]
pub struct Settings {
    pub directory: PathBuf,
    pub filter: FilterSettings,
    pub output: OutputSettings,
    pub bump: BumpSettings,
    pub prerelease: PrereleaseSettings,
    pub build: BuildMetadataSettings,
}

impl Settings {
    pub fn merge(cli: &Cli, config: &Configuration) -> Self {
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

        let mut increment: u64 = default::INCREMENT;

        let mut prerelease_identifier: String = default::PRERELEASE_IDENTIFIER.to_string();
        let mut prerelease_template: String = default::PRERELEASE_TEMPLATE.to_string();

        let mut build_metadata_template: String = default::BUILD_METADATA_TEMPLATE.to_string();
        match &cli.cmd {
            crate::cli::Commands::Current { field: _ } => {}
            crate::cli::Commands::Next { strategy, field: _ } => match strategy {
                Some(strategy) => match strategy {
                    Strategy::Major(StandardBumpArgs {
                        bump_options,
                        build_metadata_options,
                    }) => {
                        increment = Settings::get_increment(config, bump_options, command);
                        build_metadata_template = Settings::get_build_metadata_template(
                            config,
                            build_metadata_options,
                            command,
                        );
                    }
                    Strategy::Minor(StandardBumpArgs {
                        bump_options,
                        build_metadata_options,
                    }) => {
                        increment = Settings::get_increment(config, bump_options, command);
                        build_metadata_template = Settings::get_build_metadata_template(
                            config,
                            build_metadata_options,
                            command,
                        );
                    }
                    Strategy::Patch(StandardBumpArgs {
                        bump_options,
                        build_metadata_options,
                    }) => {
                        increment = Settings::get_increment(config, bump_options, command);
                        build_metadata_template = Settings::get_build_metadata_template(
                            config,
                            build_metadata_options,
                            command,
                        );
                    }
                    Strategy::Prerelease(PrereleaseArgs {
                        prerelease_options,
                        build_metadata_options,
                    }) => {
                        prerelease_identifier = Settings::get_prerelease_identifier(
                            config,
                            prerelease_options,
                            command,
                        );
                        prerelease_template =
                            Settings::get_prerelease_template(config, prerelease_options, command);
                        build_metadata_template = Settings::get_build_metadata_template(
                            config,
                            build_metadata_options,
                            command,
                        );
                    }
                    Strategy::PreMajor(PreReleaseWithBumpArgs {
                        bump_options,
                        prerelease_options,
                        build_metadata_options,
                    }) => {
                        increment = Settings::get_increment(config, bump_options, command);
                        prerelease_identifier = Settings::get_prerelease_identifier(
                            config,
                            prerelease_options,
                            command,
                        );
                        prerelease_template =
                            Settings::get_prerelease_template(config, prerelease_options, command);
                        build_metadata_template = Settings::get_build_metadata_template(
                            config,
                            build_metadata_options,
                            command,
                        );
                    }
                    Strategy::PreMinor(PreReleaseWithBumpArgs {
                        bump_options,
                        prerelease_options,
                        build_metadata_options,
                    }) => {
                        increment = Settings::get_increment(config, bump_options, command);
                        prerelease_identifier = Settings::get_prerelease_identifier(
                            config,
                            prerelease_options,
                            command,
                        );
                        prerelease_template =
                            Settings::get_prerelease_template(config, prerelease_options, command);
                        build_metadata_template = Settings::get_build_metadata_template(
                            config,
                            build_metadata_options,
                            command,
                        );
                    }
                    Strategy::PrePatch(PreReleaseWithBumpArgs {
                        bump_options,
                        build_metadata_options,
                        prerelease_options,
                    }) => {
                        increment = Settings::get_increment(config, bump_options, command);
                        prerelease_identifier = Settings::get_prerelease_identifier(
                            config,
                            prerelease_options,
                            command,
                        );
                        prerelease_template =
                            Settings::get_prerelease_template(config, prerelease_options, command);
                        build_metadata_template = Settings::get_build_metadata_template(
                            config,
                            build_metadata_options,
                            command,
                        );
                    }
                    Strategy::Dev(PrereleaseArgs {
                        prerelease_options,
                        build_metadata_options,
                    }) => {
                        prerelease_identifier = Settings::get_prerelease_identifier(
                            config,
                            prerelease_options,
                            command,
                        );
                        prerelease_template =
                            Settings::get_prerelease_template(config, prerelease_options, command);
                        build_metadata_template = Settings::get_build_metadata_template(
                            config,
                            build_metadata_options,
                            command,
                        );
                    }
                },
                None => {}
            },
        }

        Self {
            directory,
            filter: FilterSettings { tag: filter_tag },
            output: OutputSettings {
                format: output_format,
                template: output_template,
            },
            bump: BumpSettings {
                increment: increment,
            },
            prerelease: PrereleaseSettings {
                identifier: prerelease_identifier,
                template: prerelease_template,
            },
            build: BuildMetadataSettings {
                template: build_metadata_template,
            },
        }
    }

    fn get_increment(config: &Configuration, bump_options: &BumpingOptions, command: &str) -> u64 {
        match bump_options.increment {
            Some(i) => i,
            None => match config.get::<u64>(command, "increment") {
                Ok(i) => i,
                Err(_) => default::INCREMENT,
            },
        }
    }

    fn get_prerelease_identifier(
        config: &Configuration,
        prerelease_options: &PrereleaseOptions,
        command: &str,
    ) -> String {
        match &prerelease_options.identifier {
            Some(s) => s.clone(),
            None => match config.get::<String>(command, "identifier") {
                Ok(s) => s,
                Err(_) => default::PRERELEASE_IDENTIFIER.to_string(),
            },
        }
    }

    fn get_prerelease_template(
        config: &Configuration,
        prerelease_options: &PrereleaseOptions,
        command: &str,
    ) -> String {
        match &prerelease_options.prerelease_template {
            Some(s) => s.clone(),
            None => match config.get::<String>(command, "template") {
                Ok(s) => s,
                Err(_) => default::PRERELEASE_TEMPLATE.to_string(),
            },
        }
    }

    fn get_build_metadata_template(
        config: &Configuration,
        build_metadata_options: &BuildMetadataOptions,
        command: &str,
    ) -> String {
        match &build_metadata_options.build_metadata_template {
            Some(s) => s.clone(),
            None => match config.get::<String>(command, "template") {
                Ok(s) => s,
                Err(_) => default::BUILD_METADATA_TEMPLATE.to_string(),
            },
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
