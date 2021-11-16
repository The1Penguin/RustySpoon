use serenity::{
    async_trait,
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
        gateway::Ready,
        id::UserId,
        permissions::Permissions,
    },
    prelude::*,
};

use std::{
    collections::{HashMap, HashSet},
    net::{SocketAddr, TcpStream},
    time::{Duration, SystemTime},
};

use roux::User;

#[command]
pub async fn down(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let addrs = [SocketAddr::from(([195, 82, 50, 47], 54992))];
    if let Ok(_) = TcpStream::connect(&addrs[..]) {
        if let Err(why) = msg.channel_id.say(&ctx.http, "Louisoix is up").await {
            println!("Error sending message: {:?}", why);
            ()
        }
    } else {
        if let Err(why) = msg.channel_id.say(&ctx.http, "Louisoix is down").await {
            println!("Error sending message: {:?}", why);
            ()
        }
    }

    Ok(())
}

#[command]
pub async fn fashion(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let user = User::new("kaiyoko");
    let link = match user.submitted().await {
        Ok(v) => {
            let mut templink: String = "".to_owned();
            for i in v.data.children {
                if i.data.title.contains("Fashion Report - Full Details") {
                    templink = match i.data.url {
                        Some(v) => v,
                        None => return Ok(()),
                    };
                    break;
                }
            }
            templink
        }
        Err(why) => {
            println!("Error reading submitted, {}", why);
            return Ok(());
        }
    };
    if let Err(why) = msg.channel_id.say(&ctx.http, &link as &str).await {
        println!("Error sending message: {:?}", why);
        ()
    }
    return Ok(());
}
