mod cli;
mod config;
mod version;
mod settings;
mod template;

use clap::Parser;
use semver::Version;
use serde_json::{json, Value};

use cli::{Cli, Commands, Field, Format, PrereleaseOptions, Strategy};
use version::{current_version, next_version};
use settings::{default, Settings};

use git2::Repository;

fn format_version(
    field: &Option<Field>,
    version: &Version,
    output_template: &str,
    format: &Format,
) {
    match format {
        Format::Plain => match field {
            None => {
                println!(
                    "{}",
                    output_template.replace("{version}", version.to_string().as_str())
                );
            }
            Some(part) => match part {
                Field::Major => println!("{}", version.major),
                Field::Minor => println!("{}", version.minor),
                Field::Patch => println!("{}", version.patch),
                Field::Pre => println!("{}", version.pre),
                Field::Build => println!("{}", version.build),
            },
        },
        Format::Json => {
            let json_value = match field {
                Some(Field::Major) => json!({ "major": version.major }),
                Some(Field::Minor) => json!({ "minor": version.minor }),
                Some(Field::Patch) => json!({ "patch": version.patch }),
                Some(Field::Pre) => json!({ "pre": version.pre.as_str() }),
                Some(Field::Build) => json!({ "build": version.build.as_str() }),
                None => {
                    let mut map = serde_json::Map::new();
                    map.insert("major".to_string(), json!(version.major));
                    map.insert("minor".to_string(), json!(version.minor));
                    map.insert("patch".to_string(), json!(version.patch));
                    if !version.pre.is_empty() {
                        map.insert("pre".to_string(), json!(version.pre.as_str()));
                    }
                    if !version.build.is_empty() {
                        map.insert("build".to_string(), json!(version.build.as_str()));
                    }
                    map.insert("full".to_string(), json!(version.to_string()));
                    Value::Object(map)
                }
            };
            // This unwrap is generally safe if json! macro is used correctly.
            // Consider .expect("Failed to serialize to JSON") for clarity.
            println!("{}", serde_json::to_string_pretty(&json_value).unwrap());
        }
    }
}

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
