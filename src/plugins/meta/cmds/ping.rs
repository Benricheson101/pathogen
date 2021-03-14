use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};
use tokio::time::Instant;

use crate::i18n_args;

#[command("ping")]
#[description = "Pong! See how long it takes the bot to respond"]
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

    Ok(())
}
