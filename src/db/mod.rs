mod postgres;
mod redis;

use std::{error, fmt};

use chrono::{DateTime, Utc};
pub use postgres::Postgres;
use serenity::{async_trait, model::id::GuildId, prelude::TypeMapKey};

// -- TABLE MODELS --
#[derive(Debug)]
pub struct GuildConfig {
    pub id: i64,
    pub prefix: Option<String>,
}

#[derive(Debug)]
pub struct Strike {
    pub id: i32,
    pub guild_id: i64,
    pub target: i64,
    pub moderator: i64,
    pub kind: StrikeKind,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
}

#[derive(Debug, sqlx::Type)]
#[non_exhaustive]
#[sqlx(type_name = "strike_kind", rename_all = "lowercase")]
pub enum StrikeKind {
    Ban,
    Kick,
    Mute,
    Warn,
}

// -- GENERAL DB STUFF --
crate::impl_tmk![Postgres];

#[async_trait]
pub trait PathogenDb
where
    Self: Send + Sync + TypeMapKey,
{
    async fn new() -> Self;

    async fn get_guild_prefix(
        &self,
        guild_id: Option<GuildId>,
    ) -> Option<String>;

    async fn set_guild_prefix(
        &self,
        guild_id: GuildId,
        prefix: String,
    ) -> DbResult<()>;
}

// -- ERROR HANDLING STUFF --
pub type DbResult<T> = Result<T, PathogenDbError>;

#[derive(Debug)]
pub enum PathogenDbError {
    DatabaseError(sqlx::Error),
    RedisError(mobc_redis::redis::RedisError),
    RedisMobcError(mobc::Error<mobc_redis::redis::RedisError>),
}

impl error::Error for PathogenDbError {}

impl fmt::Display for PathogenDbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathogenDbError::DatabaseError(err) => {
                write!(f, "Database Error: {:#?}", err)
            },
            PathogenDbError::RedisError(err) => {
                write!(f, "Redis Error: {:#?}", err)
            },
            PathogenDbError::RedisMobcError(err) => {
                write!(f, "Redis Mobc Error: {:#?}", err)
            },
        }
    }
}

impl From<sqlx::Error> for PathogenDbError {
    fn from(err: sqlx::Error) -> Self {
        Self::DatabaseError(err)
    }
}

impl From<mobc_redis::redis::RedisError> for PathogenDbError {
    fn from(err: mobc_redis::redis::RedisError) -> Self {
        Self::RedisError(err)
    }
}

impl From<mobc::Error<mobc_redis::redis::RedisError>> for PathogenDbError {
    fn from(err: mobc::Error<mobc_redis::redis::RedisError>) -> Self {
        Self::RedisMobcError(err)
    }
}
