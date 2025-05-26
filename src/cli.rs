use std::path::PathBuf;

use clap::{
    builder::{
        styling::{Effects, RgbColor},
        Styles,
    },
    Args, Parser, Subcommand, ValueEnum,
};
use serde::{Serialize, Deserialize};

use crate::settings::default;

/// Dynamic version manager for Git repositories
#[derive(Parser, Debug, Serialize, Deserialize)]
#[clap(author, version, styles=get_styles())]
pub struct Cli {
    #[clap(short, long, value_name="PATH", help="Path to the Git repository", default_value=default::DIRECTORY)]
    pub directory: PathBuf,

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

#[derive(Subcommand, Debug, Serialize, Deserialize)]
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
        strategy: Strategy,

        /// Field/part of the version
        #[clap(short, long)]
        field: Option<Field>,
    },
}

#[derive(ValueEnum, Clone, Debug, Serialize, Deserialize)]
pub enum Field {
    Major,
    Minor,
    Patch,
    Pre,
    Build,
}

/// Bumping strategy
#[derive(Subcommand, Debug, Serialize, Deserialize)]
#[clap(
    subcommand_help_heading = "Bumping strategy",
    subcommand_value_name = "STRATEGY"
)]
pub enum Strategy {
    /// Major version
    Major {
        #[clap(flatten)]
        bump_options: BumpingOptions,
        #[clap(flatten)]
        build_metadata_options: BuildMetadataOptions,
    },
    /// Minor version
    Minor {
        #[clap(flatten)]
        bump_options: BumpingOptions,
        #[clap(flatten)]
        build_metadata_options: BuildMetadataOptions,
    },
    /// Patch version
    Patch {
        #[clap(flatten)]
        bump_options: BumpingOptions,
        #[clap(flatten)]
        build_metadata_options: BuildMetadataOptions,
    },
    /// Pre-release version
    Prerelease {
        #[clap(flatten)]
        prerelease_options: PrereleaseOptions,
        #[clap(flatten)]
        build_metadata_options: BuildMetadataOptions,
    },
    /// Major + pre-release version
    PreMajor {
        #[clap(flatten)]
        prerelease_options: PrereleaseOptions,

        #[clap(flatten)]
        bump_options: BumpingOptions,

        #[clap(flatten)]
        build_metadata_options: BuildMetadataOptions,
    },
    /// Minor + pre-release version
    PreMinor {
        #[clap(flatten)]
        prerelease_options: PrereleaseOptions,

        #[clap(flatten)]
        bump_options: BumpingOptions,

        #[clap(flatten)]
        build_metadata_options: BuildMetadataOptions,
    },
    /// Patch + pre-release version
    PrePatch {
        #[clap(flatten)]
        prerelease_options: PrereleaseOptions,

        #[clap(flatten)]
        bump_options: BumpingOptions,

        #[clap(flatten)]
        build_metadata_options: BuildMetadataOptions,
    },
    /// Development version (non-standard)
    Dev {
        #[clap(flatten)]
        prerelease_options: PrereleaseOptions,

        #[clap(flatten)]
        build_metadata_options: BuildMetadataOptions,
    },
}
impl Strategy {
    pub fn get_build_metadata_options(&self) -> &BuildMetadataOptions {
        match self {
            Strategy::Major {
                build_metadata_options,
                ..
            } => build_metadata_options,
            Strategy::Minor {
                build_metadata_options,
                ..
            } => build_metadata_options,
            Strategy::Patch {
                build_metadata_options,
                ..
            } => build_metadata_options,
            Strategy::Prerelease {
                build_metadata_options,
                ..
            } => build_metadata_options,
            Strategy::PreMajor {
                build_metadata_options,
                ..
            } => build_metadata_options,
            Strategy::PreMinor {
                build_metadata_options,
                ..
            } => build_metadata_options,
            Strategy::PrePatch {
                build_metadata_options,
                ..
            } => build_metadata_options,
            Strategy::Dev {
                build_metadata_options,
                ..
            } => build_metadata_options,
        }
    }
}

#[derive(Args, Debug, Serialize, Deserialize)]
pub struct PrereleaseOptions {
    /// Prerelease identifier (e.g., alpha, beta, build, ...)
    #[clap(default_value = default::PRERELEASE_IDENTIFIER)]
    pub identifier: String,

    /// Template for next version's pre-release
    #[clap(short, long, default_value = default::PRERELEASE_TEMPLATE)]
    pub prerelease_template: String,
}

#[derive(Args, Debug, Serialize, Deserialize)]
pub struct BumpingOptions {
    /// Bump increment
    #[clap(short, long, default_value_t = default::INCREMENT)]
    pub increment: u64,
}

#[derive(Args, Debug, Serialize, Deserialize)]
pub struct BuildMetadataOptions {
    #[clap(short, long, help ="Template for build metadata", default_value=default::BUILD_METADATA_TEMPLATE)]
    pub template: String,
}

#[derive(Debug, Args, Serialize, Deserialize)]
#[group(required = false, multiple = false)]
pub struct FilterOptions {
    #[clap(short, long, value_name="REGEX",  help="Regular expression for selecting relevant tags", default_value=default::TAG_FILTER)]
    pub tag_filter: String,
}

/// Output options
#[derive(Debug, Args, Serialize, Deserialize)]
#[group(required = false, multiple = false)]
pub struct OutputOptions {
    #[clap(short, long, help = "Output format", default_value_t=Format::Plain)]
    pub format: Format,
    #[clap(long, short, help="Template for resulting version", default_value=default::OUTPUT_TEMPLATE)]
    pub output_template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ValueEnum)]
pub enum Format {
    Plain,
    Json,
}
impl ToString for Format {
    fn to_string(&self) -> String {
        match &self {
            Format::Plain => String::from("plain"),
            Format::Json =>  String::from("json"),
        }
    }
}

fn get_styles() -> Styles {
    Styles::styled()
        .header(RgbColor::from((246, 193, 119)).on_default() | Effects::BOLD)
        .usage(RgbColor::from((196, 167, 231)).on_default() | Effects::BOLD)
        .literal(RgbColor::from((235, 188, 186)).on_default() | Effects::BOLD)
        .placeholder(RgbColor::from((196, 167, 231)).on_default())
}
