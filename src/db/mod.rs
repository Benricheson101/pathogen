mod postgres;
mod redis;

use std::{error, fmt, option};

pub use postgres::Postgres;

use crate::plugins::moderation::Strike;
// use crate::plugins::config::GuildConfig;

// -- GENERAL DB STUFF --
crate::impl_tmk![Postgres];

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
                write!(f, "Redis Connection Pool Error: {:#?}", err)
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
