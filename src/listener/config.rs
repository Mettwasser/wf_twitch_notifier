use better_default::Default;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ListenerConfigOptions {
    pub enabled: bool,
    pub format: String,
}

impl ListenerConfigOptions {
    pub fn new(s: impl Into<String>) -> Self {
        Self {
            enabled: true,
            format: s.into(),
        }
    }
}

impl From<&str> for ListenerConfigOptions {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct ListenerConfig {
    /// Eidolon Hunts
    #[default("🌙 @{channel_name}, swing yo' ass over to Cetus! It's EIDOLON TIME!".into())]
    pub eidolon_hunts: ListenerConfigOptions,

    /// S Tier Arbitrations, based on the "Arbitration Goons" Tierlist
    #[default("💰 @{channel_name}, new S-Tier Arbitration: {node} on {planet}".into())]
    pub s_tier_arbitrations: ListenerConfigOptions,

    /// Meta Relics. These are just S-Tier Arbitration maps, but fissures
    #[default("🔍 @{channel_name} New Meta Fissure detected on {node} - {difficulty}".into())]
    pub meta_relics: ListenerConfigOptions,

    /// What the name implies, Steel Path Disruption Fissures
    #[default("⚡ @{channel_name} New Steel Path Disruption Fissure detected on {node}".into())]
    pub steel_path_disruption_fissures: ListenerConfigOptions,
}
