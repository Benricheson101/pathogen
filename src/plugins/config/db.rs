use chrono::{DateTime, Utc};
use serenity::model::prelude::*;
use sqlx::{postgres::PgQueryResult, types::Json};

use super::GuildConfig;
use crate::{
    db::{DbResult, PathogenDbError},
    Database,
};

#[derive(Debug)]
pub struct GuildConfigRow {
    pub id: i32,
    pub guild_id: i64,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub created_by: i64,
    pub config: Json<GuildConfig>,
}

#[derive(Debug)]
pub struct NewGuildConfigRow {
    pub guild_id: i64,
    pub is_active: bool,
    pub created_by: i64,
    pub config: Json<GuildConfig>,
}

impl Database {
    /// Add a config for a guild.
    ///
    /// It will mark the latest config as active and mark any old configs
    /// inactive
    pub async fn add_guild_config(
        &self,
        config: &NewGuildConfigRow,
    ) -> DbResult<PgQueryResult> {
        sqlx::query!(
            r#"
            UPDATE configs
            SET is_active = false
            WHERE guild_id = $1
            "#,
            config.guild_id,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query!(
            r#"
            INSERT INTO configs (guild_id, is_active, created_by, config)
            VALUES ($1, $2, $3, $4)
            "#,
            config.guild_id,
            config.is_active,
            config.created_by,
            config.config as _,
        )
        .execute(&self.pool)
        .await
        .map_err(|err| PathogenDbError::DatabaseError(err))
    }

    /// Get the active config for a guild
    pub async fn get_active_guild_config(
        &self,
        guild_id: &GuildId,
    ) -> DbResult<GuildConfigRow> {
        sqlx::query_as!(
            GuildConfigRow,
            r#"
                SELECT
                    id as "id: i32",
                    guild_id as "guild_id: i64",
                    is_active as "is_active: bool",
                    created_at as "created_at: DateTime<Utc>",
                    created_by as "created_by: i64",
                    config as "config: Json<GuildConfig>"
                FROM configs
                WHERE guild_id = $1 AND is_active
            "#,
            guild_id.0 as i64,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| PathogenDbError::DatabaseError(err))
    }

    /// Get the prefix for a guild
    pub async fn get_guild_prefix(
        &self,
        guild_id: &Option<GuildId>,
    ) -> DbResult<String> {
        if let Some(guild_id) = guild_id {
            let query = sqlx::query!(
                r#"
                    SELECT config
                    FROM configs
                    WHERE guild_id = $1 AND is_active
                "#,
                guild_id.0 as i64,
            )
            .fetch_one(&self.pool)
            .await?;

            Ok(query.config["prefix"].to_string())
        } else {
            Ok("~".to_string())
        }
    }
}
