use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[allow(unused_variables)]
#[command("ban")]
#[description = "Ban a user from the server"]
pub async fn ban_cmd(
    ctx: &Context,
    msg: &Message,
    args: Args,
) -> CommandResult {
    // placeholder
    Ok(())
}
