use config::{
    ConfigError,
    File,
    Environment,
};
use serde_derive::Deserialize;
use std::path::PathBuf;

use super::{
    alias::Aliases,
    variable::Variables,
};

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub variables: Variables,
    
    #[serde(default)]
    pub aliases: Aliases,
}

const CONFIG_FILE_NAME: &'static str = "wrap";

impl Config {
    pub fn new(path: &Option<PathBuf>) -> Result<Self, ConfigError> {
        let mut config = config::Config::builder();

        // Start off by merging in the global configuration file
        // TODO: linux only
        config = config.add_source(File::with_name(&format!("/etc/{}", CONFIG_FILE_NAME)).required(false));

        if let Some(path) = path {
            // Use only the given config
            config = config.add_source(File::with_name(&path.to_string_lossy()).required(true));
        } else {
            // Add in the user's config
            // This doesn't use config_dir since it's a weird path on MacOS
            if let Some(path) = dirs::home_dir() {
                config = config.add_source(File::with_name(&format!("{}/.config/{}", path.to_string_lossy(), CONFIG_FILE_NAME)).required(false));
            }

            // Add in the working dir's config
            config = config.add_source(File::with_name(CONFIG_FILE_NAME).required(false));
        }

        // Add in settings from the environment (with a prefix of WRAPPER)
        // Eg.. `WRAPPER_DEBUG=1 ./target/app` would set the `debug` key
        config = config.add_source(Environment::with_prefix(CONFIG_FILE_NAME));

        // Deserialize the config
        config.build()?.try_into()
    }
}
