use std::path::PathBuf;

use clap::{
    builder::{
        styling::{Effects, RgbColor},
        Styles,
    },
    Args, Parser, Subcommand, ValueEnum,
};

use crate::settings::default;

/// Dynamic version manager for Git repositories
#[derive(Parser, Debug)]
#[clap(author, version, styles=get_styles())]
pub struct Cli {
    #[clap(short, long, value_name="PATH", help=format!("Path to the Git repository [default: {}]", default::DIRECTORY))]
    pub directory: Option<PathBuf>,

    #[clap(
        short,
        long,
        value_name = "PATH",
        help = "Path to the config file or directory"
    )]
    pub config: Option<PathBuf>,

    #[clap(flatten, next_help_heading = "Filter options")]
    pub filter_options: FilterOptions,

    #[clap(flatten, next_help_heading = "Output options")]
    pub output_options: OutputOptions,

    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Get current version
    Current {
        /// Field/part of the version
        #[clap(short, long)]
        field: Option<Field>,
    },
    /// Get next version
    Next {
        #[clap(subcommand)]
        strategy: Option<Strategy>,

        /// Field/part of the version
        #[clap(short, long)]
        field: Option<Field>,
    },
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Field {
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
pub enum Strategy {
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
    /// Development version (non-standard)
    Dev {
        #[clap(flatten)]
        prerelease_options: PrereleaseOptions,
    },
}

#[derive(Args, Debug)]
pub struct PrereleaseOptions {
    /// Prerelease identifier (e.g., alpha, beta, build, ...)
    #[clap(default_value = default::PRERELEASE_IDENTIFIER)]
    pub identifier: String,

    /// Template for next version's pre-release
    #[clap(short, long, default_value = default::PRERELEASE_TEMPLATE)]
    pub prerelease_template: String,

    /// Template for next version's build metadata
    #[clap(short, long, default_value = default::BUILD_TEMPLATE)]
    pub build_template: String,
}

#[derive(Args, Debug)]
pub struct BumpingOptions {
    /// Bump increment
    #[clap(short, long, default_value_t = default::INCREMENT)]
    pub increment: u64,
}

#[derive(Debug, Args)]
#[group(required = false, multiple = false)]
pub struct FilterOptions {
    #[clap(short, long, value_name="REGEX",  help=format!("Regular expression for selecting relevant tags [default: {}]", default::TAG_FILTER))]
    pub tag_filter: Option<String>,
}

/// Output options
#[derive(Debug, Args)]
#[group(required = false, multiple = false)]
pub struct OutputOptions {
    #[clap(long, short, help=format!("Template for resulting version [default: {}]", default::OUTPUT_TEMPLATE))]
    pub output_template: Option<String>,
    #[clap(short, long, help = "Template for build metadata [default: ]")]
    pub metadata_template: Option<String>,
    #[clap(short, long, help = "Output format [default: plain]")]
    pub format: Option<Format>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Format {
    Plain,
    Json,
}

fn get_styles() -> Styles {
    Styles::styled()
        .header(RgbColor::from((246, 193, 119)).on_default() | Effects::BOLD)
        .usage(RgbColor::from((196, 167, 231)).on_default() | Effects::BOLD)
        .literal(RgbColor::from((235, 188, 186)).on_default() | Effects::BOLD)
        .placeholder(RgbColor::from((196, 167, 231)).on_default())
}
