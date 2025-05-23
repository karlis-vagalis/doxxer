use std::path::PathBuf;

use config::Config;
use serde::Deserialize;

use crate::settings::default;

#[derive(Debug)]
pub struct Configuration {
    config: Config,
}
impl Configuration {
    pub fn load(config_path: Option<&PathBuf>) -> Self {
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
        Self {
            config: config.build().expect("Failed to load config"),
        }
    }

    pub fn get<'de, T>(&self, subcommand: &str, key: &str) -> Result<T, config::ConfigError>
    where
        T: Deserialize<'de>,
    {
        let parts: Vec<&str> = subcommand.split('.').collect();

        // Try from the most specific to the least specific subcommand prefix
        for i in (0..=parts.len()).rev() {
            if i > 0 {
                let prefix_parts = &parts[0..i];
                let prefix = prefix_parts.join(".");
                let env_prefix = prefix_parts.join("_").to_uppercase();

                let env_key = format!("{}_{}", env_prefix, key.to_uppercase());
                if let Ok(value) = self.config.get::<T>(&env_key) {
                    return Ok(value);
                }

                let section_key = format!("{}.{}", prefix, key);
                if let Ok(value) = self.config.get::<T>(&section_key) {
                    return Ok(value);
                }
            }
        }
        // Finally, try the global key
        self.config.get(key)
    }
}
