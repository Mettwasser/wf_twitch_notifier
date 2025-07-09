use std::fs;

use anyhow::Context;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageConfig {
    pub enabled: bool,
    pub format: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub eidolon_hunts: MessageConfig,

    pub s_tier_arbitrations: MessageConfig,

    pub meta_relics: MessageConfig,

    pub steel_path_disruption_fissures: MessageConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            eidolon_hunts: MessageConfig {
                enabled: true,
                format: "🌙 @{channel_name}, swing yo' ass over to Cetus! It's EIDOLON TIME!"
                    .to_owned(),
            },
            s_tier_arbitrations: MessageConfig {
                enabled: true,
                format: "💰 @{channel_name}, new S-Tier Arbitration: {node} on {planet}".to_owned(),
            },
            meta_relics: MessageConfig {
                enabled: true,
                format: "🔍 @{channel_name} New Meta Fissure detected on {node} - {difficulty}"
                    .to_owned(),
            },
            steel_path_disruption_fissures: MessageConfig {
                enabled: true,
                format: "⚡ @{channel_name} New Steel Path Disruption Fissure detected on {node}"
                    .to_owned(),
            },
        }
    }
}

impl Config {
    fn load_or_create() -> anyhow::Result<Self> {
        match fs::read_to_string("./config.json") {
            Ok(config) => Ok(serde_json::from_str(&config).context("Failed to parse config.json")?),
            Err(_) => {
                let config = Config::default();
                fs::write("./config.json", serde_json::to_string_pretty(&config)?)?;

                Ok(config)
            }
        }
    }

    pub fn load() -> anyhow::Result<Self> {
        Self::load_or_create()
    }
}
