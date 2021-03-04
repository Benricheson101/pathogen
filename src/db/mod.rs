mod postgres;
mod redis;

use std::{error, fmt};

pub use postgres::Postgres;
use serenity::{async_trait, model::id::GuildId, prelude::TypeMapKey};

crate::impl_tmk![Postgres];

pub type DbResult<T> = Result<T, PathogenDbError>;

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

#[derive(Debug)]
pub enum PathogenDbError {
    DatabaseError(sqlx::Error),
    RedisError(mobc::Error<mobc_redis::redis::RedisError>),
}

impl error::Error for PathogenDbError {}

impl fmt::Display for PathogenDbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathogenDbError::RedisError(err) => {
                write!(f, "Redis Error: {:#?}", err)
            },
            PathogenDbError::DatabaseError(err) => {
                write!(f, "Database Error: {:#?}", err)
            },
        }
    }
}

impl From<mobc::Error<mobc_redis::redis::RedisError>> for PathogenDbError {
    fn from(err: mobc::Error<mobc_redis::redis::RedisError>) -> Self {
        Self::RedisError(err)
    }
}

impl From<sqlx::Error> for PathogenDbError {
    fn from(err: sqlx::Error) -> Self {
        Self::DatabaseError(err)
    }
}
