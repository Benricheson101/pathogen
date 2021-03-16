use std::convert::{TryFrom, TryInto};

use if_chain::if_chain;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
};
use sqlx::types::Json;

use crate::{
    plugins::config::{GuildConfig, NewGuildConfigRow},
    regex::parse_code_block,
    I18n,
};

#[command("config")]
#[aliases("cfg", "conf", "setup")]
#[sub_commands(config_load_subcmd, config_patch_subcmd, config_get_subcmd)]
pub async fn config_cmd(_ctx: &Context, _msg: &Message) -> CommandResult {
    Ok(())
}

#[command("load")]
#[aliases("import")]
#[description = "Overwrite the guild configuration. Note: this will be reversable, just not yet :P"]
#[min_args(1)]
#[only_in(guilds)]
async fn config_load_subcmd(
    ctx: &Context,
    msg: &Message,
    args: Args,
) -> CommandResult {
    let i18n = I18n::get_from_typemap(&ctx).await;

    if let Some(guild_id) = msg.guild_id {
        let cfg = parse_code_block(args.rest().into());

        if let Ok(cfg) = GuildConfig::try_from(&cfg) {
            let data_read = ctx.data.read().await;
            let db = data_read.get::<crate::Database>().unwrap().lock().await;

            let row = NewGuildConfigRow {
                guild_id: guild_id.0 as i64,
                is_active: true,
                created_by: msg.author.id.0 as i64,
                config: Json(cfg),
            };

            let res = db.add_guild_config(&row).await?;

            println!("{:#?}", res);

            // let m = i18n.get(
            //     &msg.guild_id,
            //     if res.rows_affected() == 1 {
            //         "config-cmd-config-load-success"
            //     } else {
            //         "config-cmd-config-load-failure"
            //     },
            // ).await;

            // msg.channel_id.say(&ctx.http, m).await?;
        } else {
            msg.channel_id
                .say(
                    &ctx.http,
                    i18n.get(&msg.guild_id, "config-cmd-config-parse-error")
                        .await,
                )
                .await?;
        }
    }

    Ok(())
}

#[command("patch")]
#[aliases("modify")]
#[description = "Modify the current guild configuration. New and existing configurations are merged, so only new values are overwritten"]
#[min_args(1)]
async fn config_patch_subcmd(
    ctx: &Context,
    msg: &Message,
    args: Args,
) -> CommandResult {
    Ok(())
}

#[command("get")]
#[min_args(0)]
#[max_args(0)]
async fn config_get_subcmd(ctx: &Context, msg: &Message) -> CommandResult {
    let example_config = r#"prefix = "~"
        lang = "en""#;

    let conf = GuildConfig::try_from(&example_config.to_string()).unwrap();
    let ser: String = conf.try_into().unwrap();

    msg.channel_id
        .say(
            &ctx.http,
            format!("Sample parsed config: ```toml\n{}```", ser),
        )
        .await?;

    Ok(())
}

#[command("delete")]
#[aliases("drop")]
#[min_args(0)]
#[max_args(0)]
async fn config_delete_subcmd(ctx: &Context, msg: &Message) -> CommandResult {
    Ok(())
}
