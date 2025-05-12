mod git;

use clap::{Parser, Subcommand};
use semver::Version;
use std::path::PathBuf;

use git::handle_version;

use git2::Repository;

#[derive(Parser, Debug)]
#[clap(version)]
struct Cli {
    #[clap(short, long, default_value = ".")]
    directory: PathBuf,

    #[clap(short, long, default_value = "v")]
    prefix: Option<String>,

    #[command(subcommand)]
    cmd: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Version,
    #[clap(name = "self")]
    Doxxer {
        #[command(subcommand)]
        cmd: DoxxerCommands,
    },
}

#[derive(Subcommand, Debug)]
enum DoxxerCommands {
    Info {
        #[clap(short, long)]
        verbose: bool,
    },
    Version,
}

fn main() {
    let args = Cli::parse();

    let repo = match Repository::open(&args.directory) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    match &args.cmd {
        None => {
            handle_version(&repo, &args);
        }
        Some(command) => {
            match command {
                Commands::Version => {},
                Commands::Doxxer { cmd } => {}
            }
        }
    }

    dbg!(args);
}
