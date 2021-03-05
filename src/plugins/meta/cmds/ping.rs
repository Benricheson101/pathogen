use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};
use tokio::time::Instant;

#[command("ping")]
#[description = "Pong! See how long it takes the bot to respond"]
pub async fn ping_cmd(ctx: &Context, msg: &Message) -> CommandResult {
    let before = Instant::now();
    let mut m = msg.channel_id.say(&ctx.http, ":ping_pong: Pong!").await?;
    let elapsed = before.elapsed().as_millis();

    m.edit(&ctx, |c| {
        c.content(&format!(":ping_pong: Pong! Message sent in {}ms", elapsed))
    })
    .await?;

    Ok(())
}
