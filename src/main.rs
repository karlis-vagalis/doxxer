mod git;

use clap::{Args, Parser, Subcommand, ValueEnum};
use semver::Version;
use std::{fmt, path::PathBuf};

use git::{current_version, next_version};

use git2::Repository;

/// Dynamic version manager for Git
#[derive(Parser, Debug)]
#[clap(version, color = clap::ColorChoice::Auto)]
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
        #[command(subcommand)]
        cmd: Option<PartCommands>,
    },
    /// Returns next version string
    Next {
        /// Bumping strategy
        #[clap(short, long, value_parser = clap::value_parser!(Strategy), default_value_t)]
        strategy: Strategy,

        /// Template for next version's pre-release
        #[clap(short, long, default_value = "{pre}.dev.{distance}")]
        pre_template: String,

        /// Template for next version's build metadata
        #[clap(short, long, default_value = "{hash}")]
        build_template: String,

        #[command(subcommand)]
        cmd: Option<PartCommands>,
    },
}

#[derive(Subcommand, Debug)]
enum PartCommands {
    /// Get major version
    Major,
    /// Get minor version
    Minor,
    /// Get patch version
    Patch,
    /// Get pre-release version
    Pre,
    /// Get build metadata
    Build,
}

#[derive(ValueEnum, Clone, Debug, Default)]
enum Strategy {
    Major,
    Minor,
    Patch,
    #[default]
    PreBuild,
}
impl fmt::Display for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Strategy::Major => write!(f, "major"),
            Strategy::Minor => write!(f, "minor"),
            Strategy::Patch => write!(f, "patch"),
            Strategy::PreBuild => write!(f, "pre-build"),
        }
    }
}

/// Output options
#[derive(Debug, Args)]
#[group(required = false, multiple = false)]
struct OutputOptions {
    /// Add tag prefix to the output version
    #[clap(long, short, default_value = "v")]
    prefix: String,
}

fn output_version(cmd: &Option<PartCommands>, version: &Version, output_prefix: &str) {
    match cmd {
        None => {
            println!("{}{}", output_prefix, version);
        }
        Some(part) => match part {
            PartCommands::Major => println!("{}", version.major),
            PartCommands::Minor => println!("{}", version.minor),
            PartCommands::Patch => println!("{}", version.patch),
            PartCommands::Pre => println!("{}", version.pre),
            PartCommands::Build => println!("{}", version.build),
        },
    }
}

fn main() {
    let args = Cli::parse();

    let repo = match Repository::open(&args.directory) {
        Ok(repo) => repo,
        Err(e) => panic!("Issue opening repository! {}", e),
    };

    match &args.cmd {
        Commands::Current { cmd } => {
            let version = current_version(&repo, args.tag_prefix.as_str());
            output_version(cmd, &version, &args.version_output_options.prefix)
        }
        Commands::Next {
            cmd,
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
            output_version(cmd, &version, &args.version_output_options.prefix)
        }
    }
}
