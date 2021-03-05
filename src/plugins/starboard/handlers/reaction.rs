use serenity::{client::Context, model::channel::Reaction};

pub async fn on_reaction_add(_ctx: &Context, reaction: &Reaction) {
    let user = reaction.user_id.unwrap();
    println!("{} added reaction {}", user, reaction.emoji);
}

pub async fn on_reaction_remove(_ctx: &Context, reaction: &Reaction) {
    let user = reaction.user_id.unwrap();
    println!("{} removed reaction {}", user, reaction.emoji);
}
