use std::{
    fmt::Display,
    sync::Arc,
};

pub trait Placeholder: Send + Sync {
    fn key(&self) -> &'static str;
    fn value(&self) -> &str;
}

pub fn apply_placeholders<P: Placeholder>(
    fmt: &str,
    placeholders: impl IntoIterator<Item = P>,
) -> String {
    let mut result = fmt.to_string();

    for placeholder in placeholders {
        result = result.replace(placeholder.key(), placeholder.value());
    }

    result
}

impl<P: ?Sized + Placeholder> Placeholder for &P {
    fn key(&self) -> &'static str {
        P::key(self)
    }
    fn value(&self) -> &str {
        P::value(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChannelName(pub Arc<str>);

impl Placeholder for ChannelName {
    fn key(&self) -> &'static str {
        "{channel_name}"
    }

    fn value(&self) -> &str {
        &self.0
    }
}

impl From<String> for ChannelName {
    fn from(value: String) -> Self {
        Self(Arc::from(value))
    }
}

impl Display for ChannelName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Node<'a>(pub &'a str);

impl<'a> Placeholder for Node<'a> {
    fn key(&self) -> &'static str {
        "{node}"
    }

    fn value(&self) -> &'a str {
        self.0
    }
}

pub struct Planet<'a>(pub &'a str);

impl<'a> Placeholder for Planet<'a> {
    fn key(&self) -> &'static str {
        "{planet}"
    }

    fn value(&self) -> &'a str {
        self.0
    }
}

pub struct Difficulty(pub bool);

impl Placeholder for Difficulty {
    fn key(&self) -> &'static str {
        "{difficulty}"
    }

    fn value(&self) -> &str {
        if self.0 { "Steel Path" } else { "Normal" }
    }
}
