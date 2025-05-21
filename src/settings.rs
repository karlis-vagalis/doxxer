use std::path::PathBuf;

use config::Config;

use crate::{Cli, FilterOptions, OutputOptions};

pub mod default {
    pub static DIRECTORY: &str = ".";
    pub static FILTER_PREFIX: &str = "v";
    pub static OUTPUT_PREFIX: &str = "v";
    pub static PRERELEASE_TEMPLATE: &str = "{identifier}.{inc}";
    pub static DEV_TEMPLATE: &str = "{pre}.{identifier}.{distance}";
    pub static BUILD_TEMPLATE: &str = "{hash}";
    pub static PRERELEASE_IDENTIFIER: &str = "build";
    pub static DEV_IDENTIFIER: &str = "dev";
    pub static INCREMENT: u64 = 1;
}

#[derive(Debug)]
pub struct Settings {
    pub directory: PathBuf,
    pub filter_prefix: String,
    pub output_prefix: String,
}
impl Default for Settings{
    fn default() -> Self {
        let config = Config::builder()
            .add_source(config::File::with_name(".doxxer").required(false))
            .add_source(config::File::with_name("doxxer").required(false))
            .add_source(config::Environment::with_prefix("DOXXER"))
            .build()
            .unwrap();

        let directory = match config.get_string("directory") {
            Ok(path) => PathBuf::from(path),
            Err(_) => PathBuf::from(default::DIRECTORY),
        };
        let filter_prefix = match config.get_string("filter_prefix") {
            Ok(prefix) => prefix,
            Err(_) => default::FILTER_PREFIX.to_string(),
        };
        let output_prefix = match config.get_string("output_prefix") {
            Ok(prefix) => prefix,
            Err(_) => default::OUTPUT_PREFIX.to_string(),
        };

        Self { directory, filter_prefix, output_prefix }
    }
}
impl Settings {
    pub fn apply(&mut self, args: &Cli) {
        if let Some(directory) = &args.directory {
            self.directory = directory.clone();
        };
        if let Some(prefix) = &args.filter_options.filter_prefix {
            self.filter_prefix = prefix.clone();
        };
        if let Some(prefix) = &args.output_options.output_prefix {
            self.output_prefix = prefix.clone();
        };
        //dbg!(&self);
    }
}