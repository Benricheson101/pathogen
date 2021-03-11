use std::{convert::TryFrom, error, fmt};

use chrono::{DateTime, Utc};

pub mod cmds;

// -- DB MODELS --
/// A struct representing a row of the `strikes` table
#[derive(Debug, PartialEq)]
pub struct Strike {
    /// An automatically asigned strike ID number
    pub id: i32,
    /// The ID of the guild the strike occurred in
    pub guild_id: i64,
    /// The user the strike was given to
    pub target: i64,
    /// The moderator responsible for giving the strike
    pub moderator: i64,
    /// The action that was taken
    pub kind: StrikeKind,
    /// The reason the strike was given
    pub reason: Option<String>,
    /// How much weight the strike holds. Useful for automod threshold
    pub weight: i32,
    /// When the strike was created
    pub created_at: DateTime<Utc>,
    /// The timestamp of the most recent edit to the strike
    pub edited_at: Option<DateTime<Utc>>,
    /// Whether or not the strike is still active (not expired)
    pub active: bool,
    /// When the strike should expire
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, sqlx::Type, PartialEq)]
#[non_exhaustive]
#[sqlx(type_name = "strike_kind", rename_all = "lowercase")]
pub enum StrikeKind {
    Ban,
    Kick,
    Mute,
    Warn,
}

impl TryFrom<&str> for StrikeKind {
    type Error = StrikeError;

    fn try_from(kind: &str) -> Result<Self, Self::Error> {
        match kind.to_lowercase().as_str() {
            "ban" => Ok(StrikeKind::Ban),
            "kick" => Ok(StrikeKind::Kick),
            "mute" => Ok(StrikeKind::Mute),
            "warn" => Ok(StrikeKind::Warn),
            &_ => Err(StrikeError::UnknownKind(kind.into())),
        }
    }
}

// -- ERROR HANDLING STUFF --
/// Errors that could occur with strikes
#[derive(Debug, PartialEq)]
pub enum StrikeError {
    UnknownKind(String),
}

impl error::Error for StrikeError {}

impl fmt::Display for StrikeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StrikeError::UnknownKind(kind) => {
                write!(f, "Unknown strike kind: {}", kind)
            },
        }
    }
}
