mod git;

use clap::{Parser, Subcommand};
use semver::Version;
use std::path::PathBuf;

use git::{current_version, next_version};

use git2::{build, Repository};

/// Dynamic version manager for Git
#[derive(Parser, Debug)]
#[clap(version)]
struct Cli {
    #[clap(short, long, default_value = ".")]
    directory: PathBuf,

    #[clap(short, long, default_value = "v")]
    tag_prefix: String,

    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Current SemVer version string from latest tag
    Current {
        #[command(subcommand)]
        cmd: Option<PartCommands>,
    },
    /// Upcoming SemVer version string
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
    /// Manage the doxxer executable
    #[clap(name = "self")]
    Doxxer {
        #[command(subcommand)]
        cmd: DoxxerCommands,
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

#[derive(Subcommand, Debug)]
enum DoxxerCommands {
    Info {
        #[clap(short, long)]
        verbose: bool,
    },
    Version,
}

fn output_version(cmd: &Option<PartCommands>, version: &Version) {
    match cmd {
        None => {
            println!("{}", version);
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
        Err(e) => panic!("failed to open: {}", e),
    };

    match &args.cmd {
        Commands::Current { cmd } => {
            let version = current_version(&repo, args.tag_prefix.as_str());
            output_version(cmd, &version)
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
            output_version(cmd, &version)
        }
        Commands::Doxxer { cmd } => {}
    }
}
