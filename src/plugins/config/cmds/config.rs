use std::convert::{TryFrom, TryInto};

use if_chain::if_chain;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
};

use crate::{plugins::config::GuildConfig, I18n};

#[command("config")]
#[aliases("cfg", "conf", "setup")]
#[sub_commands(config_load_subcmd, config_get_subcmd)]
pub async fn config_cmd(_ctx: &Context, _msg: &Message) -> CommandResult {
    Ok(())
}

#[command("load")]
#[aliases("import")]
#[min_args(1)]
async fn config_load_subcmd(
    ctx: &Context,
    msg: &Message,
    args: Args,
) -> CommandResult {
    let i18n = I18n::get_from_typemap(&ctx).await;

    if_chain! {
        if let Ok(Some(caps)) = crate::regex::CODE_BLOCK.captures(&args.rest());
        if let Some(code) = caps.name("code");
        if let Ok(cfg) = GuildConfig::try_from(&code.as_str().to_string());
        then {
            msg.channel_id
                .say(
                    &ctx.http,
                    format!("parsed config: ```rs\n{:#?}\n```", &cfg),
                )
                .await?;
        } else {
            msg.channel_id
                .say(
                    &ctx.http,
                    i18n.get(
                        &msg.guild_id,
                        "config-cmd-config-parse-error",
                    )
                    .await,
                )
                .await?;
        }
    }

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
