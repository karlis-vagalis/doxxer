use std::path::PathBuf;

use config::Config;

use crate::{Cli, FilterOptions, OutputOptions};

pub mod default {
    pub static FILTER_PREFIX: &str = "v";
    pub static OUTPUT_PREFIX: &str = "v";
    pub static PRERELEASE_TEMPLATE: &str = "{identifier}.{inc}";
    pub static DEV_TEMPLATE: &str = "{pre}.{identifier}.{distance}";
    pub static BUILD_TEMPLATE: &str = "{hash}";
    pub static PRERELEASE_IDENTIFIER: &str = "build";
    pub static DEV_IDENTIFIER: &str = "dev";
    pub static INCREMENT: u64 = 1;
}

pub fn get_config() -> Config {
    Config::builder()
        .add_source(config::File::with_name(".doxxer").required(false))
        .add_source(config::File::with_name("doxxer").required(false))
        .add_source(config::Environment::with_prefix("DOXXER"))
        .build()
        .unwrap()
}

pub fn apply_config(mut args: Cli, settings: Config) -> Cli {

    dbg!(&settings);
    
    if let Ok(s) = settings.get_string("directory") {
        args.directory = PathBuf::from(s)
    }
    if let Ok(s) = settings.get_string("filter_prefix") {
        args.filter_options = FilterOptions {filter_prefix: s}
    }
    if let Ok(s) = settings.get_string("output_prefix") {
        args.version_output_options = OutputOptions {output_prefix: s}
    }

    args
}
