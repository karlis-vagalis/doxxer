use std::path::PathBuf;

use config::Config;

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

    pub fn find<T>(&self) -> Option<T> {
        None
    }
}
