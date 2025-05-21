mod git;
mod settings;

use clap::builder::styling::{Effects, RgbColor, Styles};
use clap::{Args, Parser, Subcommand, ValueEnum};
use semver::Version;
use std::path::PathBuf;

use git::{current_version, next_version};
use settings::{apply_config, default, get_config, Settings};

use git2::Repository;

/// Dynamic version manager for Git
#[derive(Parser, Debug)]
#[clap(author, version, color = clap::ColorChoice::Auto, styles=get_styles())]
struct Cli {
    /// Path to the Git repository
    #[clap(short, long)]
    directory: Option<PathBuf>,

    #[clap(flatten, next_help_heading = "Filter options")]
    filter_options: FilterOptions,

    #[clap(flatten, next_help_heading = "Output options")]
    version_output_options: OutputOptions,

    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Get current version
    Current {
        /// Field/part of the version
        #[clap(short, long)]
        field: Option<Field>,
    },
    #[clap(about=format!("Get next version\nIf no strategy is provided, falls back to dynamic version template \"{}\" with \"identifier={}\"", default::DEV_TEMPLATE, default::DEV_IDENTIFIER))]
    Next {
        #[clap(subcommand)]
        strategy: Option<Strategy>,

        /// Field/part of the version
        #[clap(short, long)]
        field: Option<Field>,
    },
}

#[derive(ValueEnum, Clone, Debug)]
enum Field {
    Major,
    Minor,
    Patch,
    Pre,
    Build,
}

/// Bumping strategy
#[derive(Subcommand, Debug)]
#[clap(
    subcommand_help_heading = "Bumping strategy",
    subcommand_value_name = "STRATEGY"
)]
enum Strategy {
    /// Major version
    Major {
        #[clap(flatten)]
        bump_options: BumpingOptions,
    },
    /// Minor version
    Minor {
        #[clap(flatten)]
        bump_options: BumpingOptions,
    },
    /// Patch version
    Patch {
        #[clap(flatten)]
        bump_options: BumpingOptions,
    },
    /// Pre-release version
    Prerelease {
        #[clap(flatten)]
        prerelease_options: PrereleaseOptions,
    },
    /// Major + pre-release version
    PreMajor {
        #[clap(flatten)]
        prerelease_options: PrereleaseOptions,

        #[clap(flatten)]
        bump_options: BumpingOptions,
    },
    /// Minor + pre-release version
    PreMinor {
        #[clap(flatten)]
        prerelease_options: PrereleaseOptions,

        #[clap(flatten)]
        bump_options: BumpingOptions,
    },
    /// Patch + pre-release version
    PrePatch {
        #[clap(flatten)]
        prerelease_options: PrereleaseOptions,

        #[clap(flatten)]
        bump_options: BumpingOptions,
    },
}

#[derive(Args, Debug)]
struct PrereleaseOptions {
    /// Prerelease identifier (e.g., alpha, beta, build, ...)
    #[clap(default_value = default::PRERELEASE_IDENTIFIER)]
    identifier: String,

    /// Template for next version's pre-release
    #[clap(short, long, default_value = default::PRERELEASE_TEMPLATE)]
    prerelease_template: String,

    /// Template for next version's build metadata
    #[clap(short, long, default_value = default::BUILD_TEMPLATE)]
    build_template: String,
}

#[derive(Args, Debug)]
struct BumpingOptions {
    /// Bump increment
    #[clap(short, long, default_value_t = default::INCREMENT)]
    increment: u64,
}

#[derive(Debug, Args)]
#[group(required = false, multiple = false)]
struct FilterOptions {
    /// Prefix of the tags used for current version detection
    #[clap(short, long, default_value = default::FILTER_PREFIX)]
    filter_prefix: String,
}

/// Output options
#[derive(Debug, Args)]
#[group(required = false, multiple = false)]
struct OutputOptions {
    /// Add prefix to the output version
    #[clap(long, short, default_value = default::OUTPUT_PREFIX)]
    output_prefix: String,
}

fn output_version(cmd: &Option<Field>, version: &Version, output_prefix: &str) {
    match cmd {
        None => {
            println!("{}{}", output_prefix, version);
        }
        Some(part) => match part {
            Field::Major => println!("{}", version.major),
            Field::Minor => println!("{}", version.minor),
            Field::Patch => println!("{}", version.patch),
            Field::Pre => println!("{}", version.pre),
            Field::Build => println!("{}", version.build),
        },
    }
}

fn get_styles() -> Styles {
    Styles::styled()
        .header(RgbColor::from((246, 193, 119)).on_default() | Effects::BOLD)
        .usage(RgbColor::from((196, 167, 231)).on_default() | Effects::BOLD)
        .literal(RgbColor::from((235, 188, 186)).on_default() | Effects::BOLD)
        .placeholder(RgbColor::from((196, 167, 231)).on_default())
}

fn main() {

    let mut settings = Settings::default();
    let args = Cli::parse();

    settings.apply(&args);

    let repo = match Repository::open(settings.directory) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Issue opening repository: {}!", e.message());
            std::process::exit(1);
        }
    };

    match &args.cmd {
        Commands::Current { field } => {
            let version = current_version(&repo, &settings.filter_prefix);
            output_version(field, &version, &args.version_output_options.output_prefix)
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
            let version = next_version(&repo, &settings.filter_prefix, strategy);
            output_version(field, &version, &args.version_output_options.output_prefix)
        }
    }
}
