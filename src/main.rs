mod git;

use clap::{Args, Parser, Subcommand};
use semver::Version;
use std::path::PathBuf;

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
        /// Template for next version's pre-release
        #[clap(short, long, default_value = "{old_pre}.dev.{distance}")]
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
            pre_template,
            build_template,
        } => {
            let version = next_version(
                &repo,
                args.tag_prefix.as_str(),
                pre_template.as_str(),
                build_template.as_str(),
            );
            output_version(cmd, &version, &args.version_output_options.prefix)
        }
    }
}
