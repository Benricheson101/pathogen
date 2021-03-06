pub use eval::*;
use serenity::framework::standard::macros::group;

mod eval;

#[group("Misc")]
#[commands(eval_cmd)]
#[description = "Miscellaneous commands that don't fit into any other categories"]
pub struct MiscCmds;
