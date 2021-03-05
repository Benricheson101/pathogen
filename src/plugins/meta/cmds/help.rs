use std::collections::HashSet;

use serenity::{
    framework::standard::{
        help_commands,
        macros::help,
        Args,
        CommandGroup,
        CommandResult,
        HelpOptions,
    },
    model::prelude::*,
    prelude::*,
};

#[help]
#[embed_success_colour = "#a97ccc"]
#[individual_command_tip = "To learn more about a command, pass its name as an argument"]
#[strikethrough_commands_tip_in_guild = ""]
pub async fn help_cmd(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(
        ctx,
        msg,
        args,
        help_options,
        groups,
        owners,
    )
    .await;

    Ok(())
}
