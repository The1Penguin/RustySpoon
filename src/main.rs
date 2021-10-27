mod commands;

use crate::commands::sentinel::TEMP_COMMAND;

use std::{
    collections::{HashMap, HashSet},
    env,
    fmt::Write,
    sync::Arc,
};

use serenity::{
    async_trait,
    prelude::*,
    framework::standard::{
        buckets::{LimitedFor, RevertBucket},
        help_commands,
        macros::{check, command, group, help, hook},
        Args,
        CommandGroup,
        CommandOptions,
        CommandResult,
        DispatchError,
        HelpOptions,
        Reason,
        StandardFramework,
    },
    http::Http,
    model::{
        channel::{Channel, Message},
        gateway::Ready,
        id::UserId,
        permissions::Permissions,
    },
};

struct Handler;

#[group]
#[commands(temp)]
struct Sentinel;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
    }

    // Prints successfully connected
    async fn ready(&self, _: Context, ready: Ready) {
        println!("connected!");
    }
}

#[tokio::main]
async fn main() {
    // Extract token from env variable
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

        let http = Http::new_with_token(&token);

    // We will fetch your bot's owners and id
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };


    let framework = StandardFramework::new()
        .configure(|c| c
                        .prefix("~")
                        .owners(owners))
                        .group(&SENTINEL_GROUP);

    // Creates a client and a handler
    let mut client =
        Client::builder(&token)
            .framework(framework)
            .event_handler(Handler)
            .await
            .expect("Err creating client");

    // Starts the client
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

