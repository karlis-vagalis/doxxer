use clap::{Parser, Subcommand};
use std::{path::PathBuf};

mod git;
use git::get_latest_tag;

use git2::Repository;

#[derive(Parser, Debug)]
#[clap(version)]
struct Cli {
    #[clap(short, long, default_value = ".")]
    directory: PathBuf,

    #[command(subcommand)]
    cmd: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
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
            get_latest_tag(repo);
        }
        Some(command) => {},
    }

    dbg!(args);
}
