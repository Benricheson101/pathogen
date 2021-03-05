use std::env;

use mobc::Pool;
use mobc_redis::{
    redis::{self, AsyncCommands},
    RedisConnectionManager,
};
use serenity::model::id::GuildId;

use super::DbResult;

pub struct Redis {
    pool: Pool<RedisConnectionManager>,
}

impl Redis {
    pub async fn new() -> Self {
        let url = env::var("REDIS_URL").expect("Missing `REDIS_URL`");

        let client =
            redis::Client::open(url).expect("Unable to connect to Redis");

        let manager = RedisConnectionManager::new(client);

        let pool = Pool::new(manager);

        Self { pool }
    }

    pub async fn get_guild_prefix(&self, guild_id: &GuildId) -> Option<String> {
        let mut conn = self.pool.get().await.ok()?;
        conn.hget("prefix", guild_id.0).await.ok()?
    }

    pub async fn set_guild_prefix(
        &self,
        guild_id: &GuildId,
        prefix: &str,
    ) -> DbResult<()> {
        let mut conn = self.pool.get().await?;
        let _: () = conn.hset("prefix", guild_id.0.to_string(), prefix).await?;

        Ok(())
    }
}
