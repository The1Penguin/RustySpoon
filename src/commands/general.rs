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
        id::{ChannelId, UserId},
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

use chrono::*;

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
    fashion_helper(&ctx.http, &msg.channel_id).await;
    return Ok(());
}

pub async fn fashion_helper(http: &Http, channel_id: &ChannelId) {
    let user = User::new("kaiyoko");
    let link = match user.submitted().await {
        Ok(v) => {
            let mut templink: String = "".to_owned();
            for i in v.data.children {
                if i.data.title.contains("Fashion Report - Full Details") {
                    templink = match i.data.url {
                        Some(v) => v,
                        None => return,
                    };
                    break;
                }
            }
            templink
        }
        Err(why) => {
            println!("Error reading submitted, {}", why);
            return;
        }
    };
    if let Err(why) = channel_id.say(http, &link as &str).await {
        println!("Error sending message: {:?}", why);
        ()
    }
}

pub async fn time_to_eorzea(date: chrono::DateTime<chrono::Local>) -> chrono::DateTime<Utc> {
    let eorzea: DateTime<Utc> = DateTime::from_utc(
        NaiveDateTime::from_timestamp(date.timestamp() * 3600 / 175, 0),
        Utc,
    );
    return eorzea;
}
