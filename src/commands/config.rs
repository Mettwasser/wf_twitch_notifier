use better_default::Default;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct CommandConfigOptions {
    pub enabled: bool,

    pub format: String,
}

impl From<&str> for CommandConfigOptions {
    fn from(value: &str) -> Self {
        Self {
            enabled: true,
            format: value.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct CommandConfig {
    #[default(r#"@{author} "{item_name}" average: {average} || moving average: {moving_average}"#.into())]
    pub average_command: CommandConfigOptions,
}
