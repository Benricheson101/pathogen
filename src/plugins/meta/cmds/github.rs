use std::process;

use lazy_static::lazy_static;
use regex::Regex;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command("github")]
#[description = "Get the link to the bot's GitHub repo"]
pub async fn github_cmd(ctx: &Context, msg: &Message) -> CommandResult {
    lazy_static! {
        static ref GITHUB_REGEX: Regex = Regex::new(
            r"(?:git@|https?://)github.com(?::|/)([-_a-zA-Z0-9]+)/([-_a-zA-Z0-9]+)(?:\.git)?"
        ).unwrap();
    }

    let output = process::Command::new("git")
        .args(&["remote", "get-url", "origin"])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    if stdout.is_empty() {
        msg.channel_id
            .say(
                &ctx.http,
                ":x: There is not a configured Git repo for this bot.",
            )
            .await?;
        return Ok(());
    }

    if let Some(caps) = GITHUB_REGEX.captures(&stdout) {
        let url = format!(
            "https://github.com/{}/{}",
            caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str()
        );

        msg.channel_id.say(&ctx.http, url).await?;
    } else {
        msg.channel_id
            .say(
                &ctx.http,
                "There was an error fetching the GitHub repository URL",
            )
            .await?;
    }

    Ok(())
}
