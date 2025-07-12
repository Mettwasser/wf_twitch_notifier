use std::{
    fs,
    str::FromStr,
};

use anyhow::Context;
use semver::Version;
use serde::{
    Deserialize,
    Serialize,
};

pub const CONFIG_PATH: &str = "./config.json";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Config {
    pub version: Version,
    pub message_config: MessageConfig,
    pub command_config: CommandConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct MessageConfigOptions {
    pub enabled: bool,
    pub format: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct MessageConfig {
    pub eidolon_hunts: MessageConfigOptions,

    pub s_tier_arbitrations: MessageConfigOptions,

    pub meta_relics: MessageConfigOptions,

    pub steel_path_disruption_fissures: MessageConfigOptions,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct CommandConfigOptions {
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct CommandConfig {
    pub average_command: CommandConfigOptions,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: Version::from_str(env!("CARGO_PKG_VERSION")).unwrap(),
            message_config: MessageConfig {
                eidolon_hunts: MessageConfigOptions {
                    enabled: true,
                    format: "🌙 @{channel_name}, swing yo' ass over to Cetus! It's EIDOLON TIME!"
                        .to_owned(),
                },
                s_tier_arbitrations: MessageConfigOptions {
                    enabled: true,
                    format: "💰 @{channel_name}, new S-Tier Arbitration: {node} on {planet}"
                        .to_owned(),
                },
                meta_relics: MessageConfigOptions {
                    enabled: true,
                    format: "🔍 @{channel_name} New Meta Fissure detected on {node} - {difficulty}"
                        .to_owned(),
                },
                steel_path_disruption_fissures: MessageConfigOptions {
                    enabled: true,
                    format:
                        "⚡ @{channel_name} New Steel Path Disruption Fissure detected on {node}"
                            .to_owned(),
                },
            },
            command_config: CommandConfig {
                average_command: CommandConfigOptions { enabled: true },
            },
        }
    }
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
