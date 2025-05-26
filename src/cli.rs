use std::path::PathBuf;

use clap::{
    builder::{
        styling::{Effects, RgbColor},
        Styles,
    },
    Args, Parser, Subcommand, ValueEnum,
};
use regex::Regex;

use crate::{config::Configuration, default};

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
    pub filter: FilterOptions,

    #[clap(flatten, next_help_heading = "Output options")]
    pub output: OutputOptions,

    #[command(subcommand)]
    pub cmd: Commands,
}
impl Cli {
    pub fn apply(&mut self, config: Configuration) {
        let command = match &self.cmd {
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

        self.directory = match &self.directory {
            Some(directory) => Some(directory.clone()),
            None => match config.get::<String>(command, "directory") {
                Ok(dir) => Some(PathBuf::from(dir)),
                Err(_) => Some(PathBuf::from(default::DIRECTORY)),
            },
        };

        self.filter.tag = match &self.filter.tag {
            Some(filter) => Some(filter.clone()),
            None => match config.get::<String>(command, "filter.tag") {
                Ok(tag_filter) => Some(tag_filter),
                Err(_) => Some(default::TAG_FILTER.to_string()),
            },
        };

        self.output.format = match &self.output.format {
            Some(format) => Some(format.clone()),
            None => match config.get::<String>(command, "output.format") {
                Ok(format) => Some(Format::from_str(&format, true).unwrap()),
                Err(_) => Some(Format::Plain),
            },
        };

        self.output.template = match &self.output.template {
            Some(template) => Some(template.clone()),
            None => match config.get::<String>(command, "output.template") {
                Ok(template) => Some(template),
                Err(_) => Some(default::OUTPUT_TEMPLATE.to_string()),
            },
        };
    }
    pub fn validate(&self) {
        /*
        if !self.output_template.contains("{version}") {
            eprintln!(
                "Output template \"{}\" is missing required variable {{version}}",
                self.output_template
            );
            std::process::exit(1);
        }
         */
    }
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
    Major(StandardBumpArgs),
    /// Minor version
    Minor(StandardBumpArgs),
    /// Patch version
    Patch(StandardBumpArgs),
    /// Pre-release version
    Prerelease(PrereleaseArgs),
    /// Major + pre-release version
    PreMajor(PreReleaseWithBumpArgs),
    /// Minor + pre-release version
    PreMinor(PreReleaseWithBumpArgs),
    /// Patch + pre-release version
    PrePatch(PreReleaseWithBumpArgs),
    /// Development version (non-standard)
    Dev(PrereleaseArgs),
}
#[derive(Debug, Args)]
pub struct StandardBumpArgs {
    #[clap(flatten)]
    pub bump_options: BumpingOptions,
    #[clap(flatten)]
    pub build_metadata_options: BuildMetadataOptions,
}

#[derive(Debug, Args)]
pub struct PrereleaseArgs {
    #[clap(flatten)]
    pub prerelease_options: PrereleaseOptions,
    #[clap(flatten)]
    pub build_metadata_options: BuildMetadataOptions,
}

#[derive(Debug, Args)]
pub struct PreReleaseWithBumpArgs {
    #[clap(flatten)]
    pub prerelease_options: PrereleaseOptions,
    #[clap(flatten)]
    pub bump_options: BumpingOptions,
    #[clap(flatten)]
    pub build_metadata_options: BuildMetadataOptions,
}

#[derive(Args, Debug)]
pub struct PrereleaseOptions {
    /// Prerelease identifier (e.g., alpha, beta, build, ...)
    #[clap(default_value = default::PRERELEASE_IDENTIFIER)]
    pub identifier: String,

    /// Template for next version's pre-release
    #[clap(short, long, default_value = default::PRERELEASE_TEMPLATE)]
    pub prerelease_template: String,
}

#[derive(Args, Debug)]
pub struct BumpingOptions {
    /// Bump increment
    #[clap(short, long, default_value_t = default::INCREMENT)]
    pub increment: u64,
}

#[derive(Args, Debug)]
pub struct BuildMetadataOptions {
    #[clap(short, long, help = format!("Template for build metadata [default: {}]", default::BUILD_METADATA_TEMPLATE))]
    pub template: Option<String>,
}

#[derive(Debug, Args)]
#[group(required = false, multiple = false)]
pub struct FilterOptions {
    #[clap(short, long, value_name="REGEX",  help=format!("Regular expression for selecting relevant tags [default: {}]", default::TAG_FILTER))]
    pub tag: Option<String>,
}

/// Output options
#[derive(Debug, Args)]
#[group(required = false, multiple = false)]
pub struct OutputOptions {
    #[clap(short = 'f', long, help = "Output format [default: plain]")]
    pub format: Option<Format>,
    #[clap(short='o', long, help=format!("Template for resulting version [default: {}]", default::OUTPUT_TEMPLATE))]
    pub template: Option<String>,
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
