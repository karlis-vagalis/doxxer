mod git;

use clap::{Parser, Subcommand};
use std::{path::PathBuf};

use git::get_latest_semver;

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
            match get_latest_semver(&repo, &args.prefix) {
                Ok(opt) => {
                    match opt {
                        Some(version) => {
                            dbg!(version);
                        },
                        None => todo!(),
                    }
                },
                Err(_) => todo!(),
            }
        }
        Some(command) => {},
    }

    dbg!(args);
}
