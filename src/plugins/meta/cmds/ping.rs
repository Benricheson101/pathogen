use chrono::Utc;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};
use sqlx::types::Json;
use tokio::time::Instant;

use crate::{
    i18n_args,
    plugins::config::{GuildConfig, GuildConfigRow, NewGuildConfigRow},
};

#[command("ping")]
#[description = "Pong! See how long it takes the bot to respond"]
#[owners_only]
pub async fn ping_cmd(ctx: &Context, msg: &Message) -> CommandResult {
    let i18n = crate::I18n::get_from_typemap(&ctx).await;

    let initial_msg = i18n.get(&msg.guild_id, "meta-cmd-ping-initial").await;

    let before = Instant::now();
    let mut m = msg.channel_id.say(&ctx.http, initial_msg).await?;
    let elapsed = before.elapsed().as_millis();

    let success_msg = i18n
        .replace(
            &msg.guild_id,
            "meta-cmd-ping-success",
            i18n_args! {
                "time" => elapsed,
            },
        )
        .await;

    m.edit(&ctx, |c| c.content(&success_msg)).await?;

    // let data_read = ctx.data.read().await;
    // let db = data_read.get::<crate::Database>().unwrap().lock().await;

    // let config = GuildConfig {
    //     prefix: Some("~".to_string()),
    //     lang: Some("en".to_string()),
    // };

    // let row = NewGuildConfigRow {
    //     guild_id: 579466138992508928,
    //     is_active: true,
    //     created_by: 255834596766253057,
    //     config: Json(config),
    // };

    // let result = db.add_guild_config(&row).await.unwrap();
    // println!("{:#?}", result);

    // let found = db
    //     .get_active_guild_config(&msg.guild_id.unwrap())
    //     .await
    //     .unwrap();

    // println!("{:#?}", found);

    Ok(())
}

// pub id: Option<i32>,
// pub guild_id: i64,
// pub is_active: bool,
// pub created_at: DateTime<Utc>,
// pub created_by: i64,
// pub config: Json<GuildConfig>,
