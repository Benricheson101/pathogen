use std::env;

use serenity::model::prelude::*;
use sqlx::{postgres::Postgres as SQLxPostgres, Pool};

use super::{redis::Redis, DbResult, Strike};

// TODO: setup redis
pub struct Postgres {
    pub pool: Pool<SQLxPostgres>,
    redis: Redis,
}

impl Postgres {
    pub async fn new() -> Self {
        let url = env::var("DATABASE_URL").expect("Missing `DATABASE_URL`");

        let pool = Pool::<SQLxPostgres>::connect(&url)
            .await
            .expect("Unable to connect to Postgres");

        let redis = Redis::new().await;

        Self { pool, redis }
    }


    // TODO: move to src/plugins/moderation/db.rs
    pub async fn add_strike(&self, _strike: &Strike) -> DbResult<()> {
        Ok(())
    }

    pub async fn get_all_guild_strikes(
        &self,
        _guild_id: &GuildId,
    ) -> DbResult<Option<Vec<Strike>>> {
        Ok(None)
    }

    pub async fn get_all_user_strikes(
        &self,
        _guild_id: &GuildId,
        _user: &UserId,
    ) -> DbResult<Option<Vec<Strike>>> {
        Ok(None)
    }

    pub async fn get_guild_locale(
        &self,
        _guild_id: &Option<GuildId>,
    ) -> Option<String> {
        None
    }
}
