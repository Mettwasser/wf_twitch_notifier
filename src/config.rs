use std::{
    fs,
    str::FromStr,
};

use anyhow::Context;
use better_default::Default;
use semver::Version;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    commands::config::CommandConfig,
    listener::config::ListenerConfig,
};

pub const CONFIG_PATH: &str = "./config.json";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Config {
    #[default(Version::from_str(env!("CARGO_PKG_VERSION")).unwrap())]
    pub version: Version,
    pub listener_config: ListenerConfig,
    pub command_config: CommandConfig,
}

impl Config {
    fn load_or_create() -> anyhow::Result<Self> {
        match fs::read_to_string(CONFIG_PATH) {
            Ok(config) => {
                let config: Config =
                    serde_json::from_str(&config).context("malformed config.json")?;

                let default_config = Config::default();

                if config.version != default_config.version {
                    tracing::info!(
                        "Detected different versions. Expected {}, found {}",
                        default_config.version,
                        config.version
                    );
                    tracing::info!("Initiating automatic config update...");
                    tracing::info!("Recreating config...");

                    fs::write(CONFIG_PATH, serde_json::to_string_pretty(&default_config)?)?;

                    Ok(default_config)
                } else {
                    Ok(config)
                }
            }
            Err(_) => {
                let config = Config::default();
                fs::write(CONFIG_PATH, serde_json::to_string_pretty(&config)?)?;

                Ok(config)
            }
        }
    }

    pub fn load() -> anyhow::Result<Self> {
        Self::load_or_create()
    }
}
