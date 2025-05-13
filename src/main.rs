mod git;

use clap::{Parser, Subcommand};
use semver::Version;
use std::path::PathBuf;

use git::{current_version, next_version};

use git2::Repository;

/// Dynamic version manager for Git
#[derive(Parser, Debug)]
#[clap(version)]
struct Cli {
    #[clap(short, long, default_value = ".")]
    directory: PathBuf,

    #[clap(short, long, default_value = "v")]
    prefix: Option<String>,

    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Get current version from latest tag
    Current {
        #[command(subcommand)]
        cmd: Option<PartCommands>,
    },
    /// Get upcoming version
    Next {
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
    Build
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
        },
        Some(part) => {
            match part {
                PartCommands::Major => println!("{}", version.major),
                PartCommands::Minor => println!("{}", version.minor),
                PartCommands::Patch => println!("{}", version.patch),
                PartCommands::Pre => println!("{}", version.pre),
                PartCommands::Build => println!("{}", version.build),
            }
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
        Commands::Current {cmd} => {
            let version = current_version(&repo, args.prefix.as_deref());
            output_version(cmd, &version)
        }
        Commands::Next {cmd} => {
            let version = next_version(&repo, args.prefix.as_deref());
            output_version(cmd, &version)
        }
        Commands::Doxxer { cmd } => {}
    }

    dbg!(args);
}
