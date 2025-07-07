use std::fs;

use anyhow::Context;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub eidolon_hunt_message: bool,

    pub arbitration_s_tier_message: bool,

    pub relic_meta_and_disruption_message: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            eidolon_hunt_message: true,
            arbitration_s_tier_message: true,
            relic_meta_and_disruption_message: true,
        }
    }
}

impl Config {
    fn load_or_create() -> anyhow::Result<Self> {
        match fs::read_to_string("./config.json") {
            Ok(config) => Ok(serde_json::from_str(&config).context("Failed to parse config.json")?),
            Err(_) => {
                let config = Config::default();
                fs::write("./config.json", serde_json::to_string(&config)?)?;

                Ok(config)
            }
        }
    }

    pub fn load() -> anyhow::Result<Self> {
        Self::load_or_create()
    }
}
