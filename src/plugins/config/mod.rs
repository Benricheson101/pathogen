pub mod cmds;

use std::{convert::TryFrom, fmt};

use serde::{Deserialize, Serialize};

pub trait ConfigSection<'a>
where
    Self: fmt::Debug + Deserialize<'a> + Serialize,
{
}

/// A model representing a guild's configuration.
///
/// NOTE: all fields should be `Option<T>` so they are nullable
#[derive(Debug, Deserialize, Serialize)]
pub struct GuildConfig {
    /// The prefix the bot responds to
    prefix: Option<String>,
    /// The language identifier
    lang: Option<String>,
}

impl TryFrom<&String> for GuildConfig {
    type Error = ConfigError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        toml::from_str(value).map_err(|err| ConfigError::ParseError(err))
    }
}

impl Into<String> for GuildConfig {
    fn into(self) -> String {
        toml::to_string(&self).unwrap()
    }
}
// -- ERROR HANDLING STUFF --
// pub type ConfigResult<T> = Result<T, ConfigError>;

#[derive(Debug)]
pub enum ConfigError {
    ParseError(toml::de::Error),
    SerializeError(toml::ser::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::ParseError(err) => {
                write!(f, "Error parsing config: {:#?}", err)
            },
            ConfigError::SerializeError(err) => {
                write!(f, "Error serializing config: {:#?}", err)
            },
        }
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        Self::ParseError(err)
    }
}

impl From<toml::ser::Error> for ConfigError {
    fn from(err: toml::ser::Error) -> Self {
        Self::SerializeError(err)
    }
}
