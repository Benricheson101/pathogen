pub use ban::*;
use serenity::framework::standard::macros::group;

mod ban;

#[group("Moderation")]
#[commands(ban_cmd)]
#[description = "Commands used for server moderation"]
pub struct ModerationCmds;
