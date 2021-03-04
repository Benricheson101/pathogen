mod cmds;
mod db;
mod util;

use std::{collections::HashSet, env, sync::Arc};

use cmds::meta::*;
use db::{PathogenDb, Postgres};
use dotenv::dotenv;
use serenity::{
    async_trait,
    client::{bridge::gateway::ShardManager, Context, EventHandler},
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    model::prelude::*,
    prelude::{Mutex, TypeMapKey},
    Client,
};
use tracing_subscriber::EnvFilter;

// Set the database to use here
//
// If you choose to use your own DB implementation,
// it must implement `crate::db::PathogenDb`
use_database!(Postgres);

// -- COMMAND GROUPS --
#[group("Meta")]
#[commands(ping_cmd)]
#[description = "Meta commands. Nothing too special here c:"]
struct MetaCmds;

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        println!("Ready on shard {}", ctx.shard_id);
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
                .prefix("")
                .dynamic_prefix(|ctx, msg| {
                    Box::pin(async move {
                        let data_read = ctx.data.read().await;
                        let db =
                            data_read.get::<Database>().unwrap().lock().await;

                        db.get_guild_prefix(msg.guild_id).await
                    })
                })
        })
        .help(&HELP_CMD)
        .group(&METACMDS_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<Database>(db.clone());
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
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
