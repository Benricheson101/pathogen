use serenity::{
    client::Context,
    framework::standard::{macros::hook, DispatchError},
    model::prelude::*,
};

use crate::{i18n_args, I18n};

#[hook]
pub async fn dispatch_error(
    ctx: &Context,
    msg: &Message,
    error: DispatchError,
) {
    let i18n = I18n::get_from_typemap(&ctx).await;

    match error {
        DispatchError::NotEnoughArguments { min, given } => {
            let m = i18n
                .replace(
                    &msg.guild_id,
                    "errors-incorrect-num-args",
                    i18n_args! {
                        "expected" => min,
                        "found" => given,
                    },
                )
                .await;

            msg.channel_id.say(&ctx.http, m).await.ok();
        },
        _ => (),
    }
}
