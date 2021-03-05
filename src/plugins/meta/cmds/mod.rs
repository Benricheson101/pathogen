pub use github::*;
pub use help::*;
pub use ping::*;
use serenity::framework::standard::macros::group;

mod github;
mod help;
mod ping;

#[group("Meta")]
#[commands(ping_cmd, github_cmd)]
#[description = "Meta commands. Nothing too special here c:"]
pub struct MetaCmds;
