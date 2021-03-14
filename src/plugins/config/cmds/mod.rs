pub use config::*;
use serenity::framework::standard::macros::group;

mod config;

#[group("Config")]
#[commands(config_cmd)]
#[description = "Commands related to configuration of the bot"]
#[summary = "Bot configuration"]
#[only_in(guilds)]
pub struct ConfigCmds;
