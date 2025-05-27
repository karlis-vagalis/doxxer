mod cli;
mod config;
mod settings;
mod template;
mod version;

use clap::Parser;

use cli::{BuildMetadataOptions, Cli, Commands, PrereleaseArgs, PrereleaseOptions, Strategy};
use config::Configuration;
use settings::Settings;
use version::{current_version, format_version, next_version};

use git2::Repository;

pub mod default {
    pub static CONFIG_FILE_NAME: &str = "doxxer";

    pub static DIRECTORY: &str = ".";

    pub static TAG_FILTER: &str = "";

    pub static OUTPUT_TEMPLATE: &str = "{version}";

    pub static INCREMENT: u64 = 1;

    pub static PRERELEASE_IDENTIFIER: &str = "build";
    pub static DEV_IDENTIFIER: &str = "dev";

    pub static PRERELEASE_TEMPLATE: &str = "{identifier}.{inc}";
    pub static DEV_PRERELEASE_TEMPLATE: &str = "{pre}.{identifier}.{distance}";

    pub static BUILD_METADATA_TEMPLATE: &str = "";
    pub static DEV_BUILD_METADATA_TEMPLATE: &str = "{hash}";
}

fn main() {
    let cli = Cli::parse();

    let config = match &cli.config {
        Some(config_path) => Configuration::load(Some(config_path)),
        None => match &cli.directory {
            Some(dir) => Configuration::load(Some(dir)),
            None => Configuration::load(None),
        },
    };

    let settings = Settings::merge(&cli, &config);
    settings.validate();

    let repo = match Repository::open(&settings.directory) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Issue opening repository: {}!", e.message());
            std::process::exit(1);
        }
    };

    match &cli.cmd {
        Commands::Current { field } => {
            let version = current_version(&repo, &settings.filter.tag);
            format_version(
                field,
                &version,
                &settings.output.format,
                &settings.output.template,
            )
        }
        Commands::Next { field, strategy } => {
            let strategy = match strategy {
                Some(s) => s,
                None => &Strategy::Dev(PrereleaseArgs {
                    prerelease_options: PrereleaseOptions {
                        identifier: default::DEV_IDENTIFIER.to_string(),
                        prerelease_template: default::DEV_PRERELEASE_TEMPLATE.to_string(),
                    },
                    build_metadata_options: BuildMetadataOptions {
                        template: Some(default::BUILD_METADATA_TEMPLATE.to_string()),
                    },
                }),
            };
            let version = next_version(&repo, strategy, &settings);
            format_version(
                field,
                &version,
                &settings.output.format,
                &settings.output.template,
            )
        }
    }
}
