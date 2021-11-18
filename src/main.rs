#![allow(unused_imports)]
#[macro_use]
extern crate lazy_static;
mod commands;

use commands::{general::*, sentinel::*};

use std::{
    collections::{HashMap, HashSet},
    env,
    fmt::Write,
    sync::Arc,
};

use serenity::{
    async_trait,
    client::bridge::gateway::GatewayIntents,
    framework::standard::{
        buckets::{LimitedFor, RevertBucket},
        help_commands,
        macros::{check, command, group, help, hook},
        Args, CommandGroup, CommandOptions, CommandResult, DispatchError, HelpOptions, Reason,
        StandardFramework,
    },
    http::Http,
    model::{
        channel::{Channel, Message},
        event::GuildMemberAddEvent,
        gateway::Ready,
        guild::*,
        id::{GuildId, RoleId, UserId},
        permissions::Permissions,
    },
    prelude::*,
};

struct Handler;

#[group]
#[commands(reminder, disable_reminder)]
struct Sentinel;

#[group]
#[commands(down, fashion, nodes)]
struct General;

fn get_role_id(map: HashMap<RoleId, Role>, name: String) -> Option<RoleId> {
    map.iter()
        .find_map(|(key, val)| if val.name == name { Some(*key) } else { None })
}

#[async_trait]
impl EventHandler for Handler {
    // Adds a role when a memeber joins the server
    async fn guild_member_addition(&self, ctx: Context, guild_id: GuildId, mut new_member: Member) {
        println!("Person has joined");
        let roles = match guild_id.roles(ctx.http.as_ref()).await {
            Ok(v) => v,
            Err(why) => {
                println!("Couln't get the roles: {:?}", why);
                return;
            }
        };
        let role = match get_role_id(roles, "Friends".to_string()) {
            Some(v) => v,
            None => {
                println!("Couln't find the specific role");
                return;
            }
        };

        if let Err(why) = new_member.add_role(ctx.http.as_ref(), role).await {
            println!("Adding role didn't work, {}", why);
        }
    }

    // Prints successfully connected
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} connected", ready.user.name);
        let permissions = Permissions::all();
        match ready.user.invite_url(ctx.http.as_ref(), permissions).await {
            Ok(v) => {
                println!("{} is the invite link for the bot", v);
                return;
            }
            Err(why) => {
                println!("Error getting invite url: {:?}", why);
                return;
            }
        };
    }
}

#[tokio::main]
async fn main() {
    // Extract token from env variable
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    // We will fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
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
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    node_generate();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("<").owners(owners))
        .group(&GENERAL_GROUP)
        .group(&SENTINEL_GROUP);

    // Creates a client and a handler
    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .intents(GatewayIntents::all())
        .await
        .expect("Err creating client");

    // Starts the client
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
