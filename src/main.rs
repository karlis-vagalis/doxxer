mod git;

use clap::builder::styling::{Effects, RgbColor, Styles};
use clap::{Args, Parser, Subcommand, ValueEnum};
use semver::Version;
use std::path::PathBuf;

use git::{current_version, next_version};

use git2::Repository;

/// Dynamic version manager for Git
#[derive(Parser, Debug)]
#[clap(author, version, color = clap::ColorChoice::Auto, styles=get_styles())]
struct Cli {
    /// Path to the Git repository
    #[clap(short, long, default_value = ".")]
    directory: PathBuf,

    /// Prefix of the tag names used for releases
    #[clap(short, long, default_value = "v")]
    tag_prefix: String,

    #[clap(flatten, next_help_heading = "Output options")]
    version_output_options: OutputOptions,

    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Returns current version
    Current {
        /// Field/part of the version
        #[clap(short, long)]
        field: Option<Field>,
    },
    /// Returns next version
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
    /// Bump major version
    Major {
        #[clap(flatten)]
        bump_options: BumpingOptions,
    },
    /// Bump minor version
    Minor {
        #[clap(flatten)]
        bump_options: BumpingOptions,
    },
    /// Bump patch version
    Patch {
        #[clap(flatten)]
        bump_options: BumpingOptions,
    },
    /// Bump pre-release version + build metadata [default]
    Prerelease {
        #[clap(flatten)]
        prerelease_options: PrereleaseOptions,
    },
    PreMajor {
        #[clap(flatten)]
        prerelease_options: PrereleaseOptions,
    },
    PreMinor {
        #[clap(flatten)]
        prerelease_options: PrereleaseOptions,
    },
    PrePatch {
        #[clap(flatten)]
        prerelease_options: PrereleaseOptions,
    },
}

static DEFAULT_PRERELEASE_TEMPLATE: &str = "{pre}.{identifier}.{distance}";
static DEFAULT_BUILD_TEMPLATE: &str = "{hash}";
static DEFAULT_PRERELEASE_IDENTIFIER: &str = "dev";

#[derive(Args, Debug)]
struct PrereleaseOptions {
    /// Prerelease identifier (e.g., alpha, beta, build, ...)
    #[clap(default_value = DEFAULT_PRERELEASE_IDENTIFIER)]
    identifier: String,

    /// Template for next version's pre-release
    #[clap(short, long, default_value = DEFAULT_PRERELEASE_TEMPLATE)]
    prerelease_template: String,

    /// Template for next version's build metadata
    #[clap(short, long, default_value = DEFAULT_BUILD_TEMPLATE)]
    build_template: String,
}

#[derive(Args, Debug)]
struct BumpingOptions {
    #[clap(short, long, default_value_t = 1)]
    increment: u64,
}

/// Output options
#[derive(Debug, Args)]
#[group(required = false, multiple = false)]
struct OutputOptions {
    /// Add tag prefix to the output version
    #[clap(long, short, default_value = "v")]
    prefix: String,
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
    let args = Cli::parse();

    let repo = match Repository::open(&args.directory) {
        Ok(repo) => repo,
        Err(e) => panic!("Issue opening repository! {}", e),
    };

    match &args.cmd {
        Commands::Current { field } => {
            let version = current_version(&repo, args.tag_prefix.as_str());
            output_version(field, &version, &args.version_output_options.prefix)
        }
        Commands::Next { field, strategy } => {
            let strategy = match strategy {
                Some(s) => s,
                None => &Strategy::Prerelease {
                    prerelease_options: PrereleaseOptions {
                        prerelease_template: String::from(DEFAULT_PRERELEASE_TEMPLATE),
                        build_template: String::from(DEFAULT_BUILD_TEMPLATE),
                        identifier: String::from(DEFAULT_PRERELEASE_IDENTIFIER),
                    },
                },
            };
            let version = next_version(&repo, args.tag_prefix.as_str(), strategy);
            output_version(field, &version, &args.version_output_options.prefix)
        }
    }
}
