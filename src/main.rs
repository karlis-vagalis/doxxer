mod cli;
mod config;
mod version;
mod settings;
mod template;

use clap::Parser;

use cli::{Cli, Commands, Format, PrereleaseOptions, Strategy};
use version::{current_version, format_version, next_version};
use settings::{default, Settings};

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

    let repo = match Repository::open(settings.directory) {
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
                &settings.output_template,
                &settings.output_format,
            )
        }
        Commands::Next { field, strategy } => {
            let strategy = match strategy {
                Some(s) => s,
                None => &Strategy::Prerelease {
                    prerelease_options: PrereleaseOptions {
                        prerelease_template: String::from(default::DEV_TEMPLATE),
                        build_template: String::from(default::BUILD_TEMPLATE),
                        identifier: String::from(default::DEV_IDENTIFIER),
                    },
                },
            };
            let version = next_version(&repo, &settings.tag_filter, strategy);
            format_version(
                field,
                &version,
                &settings.output_template,
                &settings.output_format,
            )
        }
    }
}
