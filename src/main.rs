mod cli;
mod config;
mod settings;
mod template;
mod version;

use clap::Parser;

use cli::{BuildMetadataOptions, Cli, Commands, Format, PrereleaseArgs, PrereleaseOptions, Strategy};
use settings::{default, Settings};
use version::{current_version, format_version, next_version};

use git2::Repository;

fn main() {
    let cli = Cli::parse();

    let mut settings = match &cli.config {
        Some(config_path) => Settings::from(config_path),
        None => match &cli.directory {
            Some(dir) => Settings::from(dir),
            None => Settings::default(),
        },
    };
    settings.apply(&cli);
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
            let version = current_version(&repo, &settings.tag_filter);
            format_version(
                field,
                &version,
                &settings,
            )
        }
        Commands::Next { field, strategy } => {
            let strategy = match strategy {
                Some(s) => s,
                None => &Strategy::Dev(PrereleaseArgs{
                    prerelease_options: PrereleaseOptions {
                        identifier: default::DEV_IDENTIFIER.to_string(),
                        prerelease_template: default::DEV_PRERELEASE_TEMPLATE.to_string(),
                    },
                    build_metadata_options: BuildMetadataOptions {
                        template: Some(default::BUILD_METADATA_TEMPLATE.to_string()),
                    },
                }),
            };
            let version = next_version(&repo, &settings.tag_filter, strategy);
            format_version(
                field,
                &version,
                &settings,
            )
        }
    }
}
