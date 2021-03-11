mod postgres;
mod redis;

use std::{error, fmt, option};

// use chrono::{DateTime, Utc};
pub use postgres::Postgres;
use serenity::{
    async_trait,
    model::id::{GuildId, UserId},
    prelude::TypeMapKey,
};

use crate::plugins::moderation::Strike;

// -- TABLE MODELS --
#[derive(Debug, PartialEq)]
pub struct GuildConfig {
    pub id: i64,
    pub prefix: Option<String>,
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
    ) -> DbResult<String>;

    async fn set_guild_prefix(
        &self,
        guild_id: GuildId,
        prefix: String,
    ) -> DbResult<()>;

    async fn add_strike(&self, strike: &Strike) -> DbResult<()>;

    async fn get_all_guild_strikes(
        &self,
        guild_id: &GuildId,
    ) -> DbResult<Option<Vec<Strike>>>;

    async fn get_all_user_strikes(
        &self,
        guild_id: &GuildId,
        user: &UserId,
    ) -> DbResult<Option<Vec<Strike>>>;
}

// -- ERROR HANDLING STUFF --
pub type DbResult<T> = Result<T, PathogenDbError>;

#[derive(Debug)]
pub enum PathogenDbError {
    DatabaseError(sqlx::Error),
    RedisError(mobc_redis::redis::RedisError),
    RedisMobcError(mobc::Error<mobc_redis::redis::RedisError>),

    NotFound(option::NoneError),
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
            PathogenDbError::NotFound(err) => {
                write!(f, "Not Found: {:#?}", err)
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

impl From<option::NoneError> for PathogenDbError {
    fn from(none: option::NoneError) -> Self {
        Self::NotFound(none)
    }
}
