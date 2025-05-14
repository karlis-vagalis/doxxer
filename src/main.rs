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
    /// Returns current version string from latest tag
    Current {
        /// Field/part of the version
        #[clap(short, long)]
        field: Option<Field>,
    },
    /// Returns next version string
    Next {
        /// Bumping strategy
        #[clap(subcommand)]
        strategy: Strategy,

        /// Field/part of the version
        #[clap(short, long)]
        field: Option<Field>,

        /// Template for next version's pre-release
        #[clap(short, long, default_value = "{pre}.dev.{distance}")]
        pre_template: String,

        /// Template for next version's build metadata
        #[clap(short, long, default_value = "{hash}")]
        build_template: String,
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

type Increment = u64;

#[derive(Subcommand, Debug, Default)]
#[clap(
    subcommand_help_heading = "Bumping strategy",
    subcommand_value_name = "STRATEGY"
)]
enum Strategy {
    /// Bump major version
    Major,
    /// Bump minor version
    Minor,
    /// Bump patch version
    Patch,
    /// Bump pre-release version + build metadata
    #[default]
    PreBuild,
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
        Commands::Next {
            field,
            strategy,
            pre_template,
            build_template,
        } => {
            let version = next_version(
                &repo,
                args.tag_prefix.as_str(),
                strategy,
                pre_template.as_str(),
                build_template.as_str(),
            );
            output_version(field, &version, &args.version_output_options.prefix)
        }
    }
}
