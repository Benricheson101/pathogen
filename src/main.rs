#![feature(try_trait)]
// Required for `std::option::NoneError`

mod db;
mod hooks;
mod plugins;
mod util;

use std::{collections::HashSet, env, sync::Arc};

use db::Postgres;
use dotenv::dotenv;
use plugins::{
    config::cmds::*,
    meta::cmds::*,
    misc::cmds::*,
    moderation::cmds::*,
    starboard,
};
use serenity::{
    async_trait,
    client::{
        bridge::gateway::{GatewayIntents, ShardManager},
        Context,
        EventHandler,
    },
    framework::StandardFramework,
    http::Http,
    model::prelude::*,
    prelude::{Mutex, TypeMapKey},
    Client,
};
use tracing_subscriber::EnvFilter;
pub use util::{i18n::*, regex};
use Postgres as Database;

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        println!("Ready on shard {}", ctx.shard_id);
    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        starboard::handlers::on_reaction_add(&ctx, &reaction).await;
    }

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        starboard::handlers::on_reaction_remove(&ctx, &reaction).await;
    }
}

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to load .env file");

    let subscriber = tracing_subscriber::fmt()
        .pretty()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to start the logger");

    let token = env::var("DISCORD_TOKEN").expect("Missing `DISCORD_TOKEN`");

    let http = Http::new_with_token(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();

            if let Some(Team { members, .. }) = info.team {
                for member in &members {
                    owners.insert(member.user.id);
                }
            } else {
                owners.insert(info.owner.id);
            }

            (owners, info.id)
        },
        Err(e) => panic!("Cound not access application info: {:?}", e),
    };

    let db = Arc::new(Mutex::new(Database::new().await));

    let framework = StandardFramework::new()
        .configure(|c| {
            c.owners(owners)
                .on_mention(Some(bot_id))
                .ignore_bots(true)
                .with_whitespace(true)
                .prefix("~")
                .dynamic_prefix(|ctx, msg| {
                    Box::pin(async move {
                        let data_read = ctx.data.read().await;
                        let db =
                            data_read.get::<Database>().unwrap().lock().await;

                        db.get_guild_prefix(&msg.guild_id).await.ok()
                    })
                })
        })
        .help(&HELP_CMD)
        .on_dispatch_error(hooks::dispatch_error)
        .group(&CONFIGCMDS_GROUP)
        .group(&METACMDS_GROUP)
        .group(&MISCCMDS_GROUP)
        .group(&MODERATIONCMDS_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .intents(
            GatewayIntents::GUILDS
                | GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::GUILD_MESSAGE_REACTIONS,
        )
        .await
        .expect("Error creating client");

    let i18n = Arc::new(I18n::new(db.clone()));

    {
        let mut data = client.data.write().await;
        data.insert::<Database>(db.clone());
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<I18n>(i18n.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl-c handler");

        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(e) = client.start_autosharded().await {
        eprintln!("Error starting client: {:#?}", e);
    }
}
