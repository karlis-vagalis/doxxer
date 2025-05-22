use regex::Regex;
use std::path::PathBuf;

use config::Config;

use crate::Cli;

pub mod default {
    pub static DIRECTORY: &str = ".";
    pub static FILTER: &str = "";
    pub static OUTPUT_TEMPLATE: &str = "{version}";
    pub static PRERELEASE_TEMPLATE: &str = "{identifier}.{inc}";
    pub static DEV_TEMPLATE: &str = "{pre}.{identifier}.{distance}";
    pub static BUILD_TEMPLATE: &str = "{hash}";
    pub static PRERELEASE_IDENTIFIER: &str = "build";
    pub static DEV_IDENTIFIER: &str = "dev";
    pub static INCREMENT: u64 = 1;
    pub static CONFIG_FILE_NAME: &str = "doxxer";
}

#[derive(Debug)]
pub struct Settings {
    pub directory: PathBuf,
    pub filter: Regex,
    pub output_template: String,
}
impl From<Config> for Settings {
    fn from(config: Config) -> Self {
        let directory = match config.get_string("directory") {
            Ok(path) => PathBuf::from(path),
            Err(_) => PathBuf::from(default::DIRECTORY),
        };
        let filter = match config.get_string("filter") {
            Ok(filter) => Regex::new(filter.as_str()).unwrap(),
            Err(_) => Regex::new(default::FILTER).unwrap(),
        };
        let output_template = match config.get_string("output_template") {
            Ok(prefix) => prefix,
            Err(_) => default::OUTPUT_TEMPLATE.to_string(),
        };

        Self {
            directory,
            filter,
            output_template,
        }
    }
}
impl Default for Settings {
    fn default() -> Self {
        let config = Settings::load_config(None);
        Settings::from(config)
    }
}
impl From<&PathBuf> for Settings {
    fn from(config_path: &PathBuf) -> Self {
        let config = Settings::load_config(Some(config_path));
        Settings::from(config)
    }
}
impl Settings {
    /// Discovers/loads configuration from specified path
    fn load_config(config_path: Option<&PathBuf>) -> Config {
        let hidden_config_file_name = format!(".{}", default::CONFIG_FILE_NAME);

        let mut config = Config::builder()
            .add_source(config::File::with_name(&hidden_config_file_name).required(false))
            .add_source(config::File::with_name(default::CONFIG_FILE_NAME).required(false));

        match config_path {
            Some(path) => {
                if path.is_file() {
                    config = config.add_source(config::File::with_name(
                        std::path::absolute(path).unwrap().to_str().unwrap(),
                    ));
                } else {
                    config = config
                        .add_source(
                            config::File::with_name(
                                path.join(&hidden_config_file_name)
                                    .as_os_str()
                                    .to_str()
                                    .unwrap(),
                            )
                            .required(false),
                        )
                        .add_source(
                            config::File::with_name(
                                path.join(default::CONFIG_FILE_NAME)
                                    .as_os_str()
                                    .to_str()
                                    .unwrap(),
                            )
                            .required(false),
                        );
                }
            }
            None => {}
        }

        config = config.add_source(config::Environment::with_prefix("DOXXER"));
        config.build().expect("Failed to load config")
    }
    pub fn apply(&mut self, args: &Cli) {
        if let Some(directory) = &args.directory {
            self.directory = directory.clone();
        };
        if let Some(filter) = &args.filter_options.filter {
            self.filter = Regex::new(filter).unwrap();
        };
        if let Some(template) = &args.output_options.output_template {
            self.output_template = template.clone();
        };
        //dbg!(&self);

        // Convert path to the absolute path
        self.directory = std::path::absolute(&self.directory).unwrap();
    }

    pub fn validate(&self) {
        if !self.output_template.contains("{version}") {
            eprintln!("Output template \"{}\" is missing required variable {{version}}", self.output_template);
            std::process::exit(1);
        }
    }
}
