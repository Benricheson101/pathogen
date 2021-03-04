mod postgres;
mod redis;

pub use postgres::Postgres;
use serenity::{
    async_trait,
    model::id::GuildId,
    prelude::TypeMapKey,
};

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

    async fn set_guild_prefix(&self, guild_id: GuildId, prefix: String);
}

pub enum PathogenDbError { // TODO
    DatabaseError,
    RedisError,
}
