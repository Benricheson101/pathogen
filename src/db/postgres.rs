use std::env;

use serenity::{async_trait, model::prelude::*};
use sqlx::{postgres::Postgres as SQLxPostgres, Pool};

use super::{redis::Redis, DbResult, PathogenDb};

pub struct Postgres {
    pool: Pool<SQLxPostgres>,
    redis: Redis,
}

#[async_trait]
impl PathogenDb for Postgres {
    async fn new() -> Self {
        let url = env::var("DATABASE_URL").expect("Missing `DATABASE_URL`");

        let pool = Pool::<SQLxPostgres>::connect(&url)
            .await
            .expect("Unable to connect to Postgres");

        let redis = Redis::new().await;

        Self { pool, redis }
    }

    async fn get_guild_prefix(
        &self,
        guild_id: Option<GuildId>,
    ) -> Option<String> {
        let guild_id = guild_id?;

        let from_redis = self.redis.get_guild_prefix(&guild_id).await;

        if from_redis.is_some() {
            from_redis
        } else {
            let query = sqlx::query!(
                "SELECT prefix FROM configs WHERE id = $1",
                guild_id.0 as i64
            )
            .fetch_one(&self.pool)
            .await;

            let prefix = query.ok()?.prefix?;

            self.redis.set_guild_prefix(&guild_id, &prefix).await;

            Some(prefix)
        }
    }

    async fn set_guild_prefix(
        &self,
        guild_id: GuildId,
        prefix: String,
    ) -> DbResult<()> {
        let result = sqlx::query!(
            r#"
            INSERT INTO configs (id, prefix)
            VALUES ($1, $2)
            ON CONFLICT (id) DO
                UPDATE SET prefix = $2
            "#,
            guild_id.0 as i64,
            prefix,
        )
        .execute(&self.pool)
        .await?;

        let redis_result =
            self.redis.set_guild_prefix(&guild_id, &prefix).await;

        println!("{:#?}\n{:#?}", result, redis_result);

        Ok(())
    }
}
